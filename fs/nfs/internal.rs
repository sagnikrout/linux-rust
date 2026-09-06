//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/internal.h
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
// NFS internal definitions
//

//
// Note: RFC 1813 doesn't limit the number of auth flavors that
// a server can return, so make something up.
//

//
// Value used if the user did not specify a port value.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_client_initdata {
    pub init_flags: c_ulong,
    pub /: *const *const *const char hostname; / Hostname of the server,
    pub /: *const *const *const sockaddr_storage addr; / Address of the server,
    pub /: *const *const *const char nodename; / Hostname of the client,
    pub /: *const *const *const char ip_addr; / IP address of the client,
    pub addrlen: usize,
    pub nfs_mod: *mut nfs_subversion,
    pub proto: c_int,
    pub minorversion: u32,
    pub nconnect: c_uint,
    pub max_connect: c_uint,
    pub net: *mut net,
    pub timeparms: *const rpc_timeout,
    pub cred: *const cred,
    pub xprtsec: xprtsec_parms,
    pub connect_timeout: c_ulong,
    pub reconnect_timeout: c_ulong,
}

//
// In-kernel mount arguments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_fs_context {
    pub internal: bool,
    pub skip_reconfig_option_check: bool,
    pub need_mount: bool,
    pub sloppy: bool,
    pub /: *mut *mut *mut unsigned int flags; / NFS{,4}_MOUNT_ flags,
    pub wsize: unsigned int rsize,,
    pub retrans: unsigned int timeo,,
    pub acregmax: unsigned int acregmin,,
    pub acdirmax: unsigned int acdirmin,,
    pub namlen: c_uint,
    pub options: c_uint,
    pub bsize: c_uint,
    pub auth_info: nfs_auth_info,
    pub selected_flavor: rpc_authflavor_t,
    pub xprtsec: xprtsec_parms,
    pub client_address: *mut c_char,
    pub version: c_uint,
    pub minorversion: c_uint,
    pub fscache_uniq: *mut c_char,
    pub protofamily: c_ushort,
    pub mountfamily: c_ushort,
    pub has_sec_mnt_opts: bool,
    pub lock_status: c_int,
    pub address: sockaddr,
    pub _address: sockaddr_storage,
}

// Information for a cloned mount.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_clone_mount {
    pub sb: *mut super_block,
    pub dentry: *mut dentry,
    pub fattr: *mut nfs_fattr,
    pub clone_data: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs_lock_status {
    NFS_LOCK_NOT_SET	= 0,
    NFS_LOCK_LOCK		= 1,
    NFS_LOCK_NOLOCK		= 2,
}

// mount_clnt.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_mount_request {
    pub sap: *mut sockaddr_storage,
    pub salen: usize,
    pub hostname: *mut c_char,
    pub dirpath: *mut c_char,
    pub version: u32,
    pub protocol: c_ushort,
    pub fh: *mut nfs_fh,
    pub noresvport: c_int,
    pub auth_flav_len: *mut c_uint,
    pub auth_flavs: *mut rpc_authflavor_t,
    pub net: *mut net,
}

extern "C" {
    pub fn nfs_mount(info: *mut nfs_mount_request, timeo: c_int, retrans: c_int) -> c_int;
}
// client.c
extern "C" {
    pub fn nfs_clients_init(net: *mut net);
}
extern "C" {
    pub fn nfs_clients_exit(net: *mut net);
}
extern "C" {
    pub fn nfs_create_rpc_client(: *mut nfs_client, : *const nfs_client_initdata, _arg: rpc_authflavor_t) -> c_int;
}
extern "C" {
    pub fn nfs_probe_server(: *mut nfs_server, : *mut nfs_fh) -> c_int;
}
extern "C" {
    pub fn nfs_server_insert_lists(: *mut nfs_server);
}
extern "C" {
    pub fn nfs_server_remove_lists(: *mut nfs_server);
}
extern "C" {
    pub fn nfs_init_timeout_values(to: *mut rpc_timeout, proto: c_int, timeo: c_int, retrans: c_int);
}
extern "C" {
    pub fn nfs_server_copy_userdata(: *mut nfs_server, : *mut nfs_server);
}
extern "C" {
    pub fn nfs_put_client(: *mut nfs_client);
}
extern "C" {
    pub fn nfs_free_client(: *mut nfs_client);
}
extern "C" {
    pub fn nfs_cb_idr_remove(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs_server_set_init_caps(: *mut nfs_server);
}
extern "C" {
    pub fn nfs_free_server(server: *mut nfs_server);
}
extern "C" {
    pub fn nfs_client_init_is_complete(clp: *const nfs_client) -> bool;
}
extern "C" {
    pub fn nfs_client_init_status(clp: *const nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs_wait_client_init_complete(clp: *const nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs_mark_client_ready(clp: *mut nfs_client, state: c_int);
}
extern "C" {
    pub fn nfs4_session_limit_rwsize(server: *mut nfs_server);
}
extern "C" {
    pub fn nfs4_session_limit_xasize(server: *mut nfs_server);
}

extern "C" {
    pub fn nfs_fs_proc_init() -> int __init;
}
extern "C" {
    pub fn nfs_fs_proc_exit();
}
extern "C" {
    pub fn nfs_fs_proc_net_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn nfs_fs_proc_net_exit(net: *mut net);
}

// callback_xdr.c
// fs_context.c
// pagelist.c
extern "C" {
    pub fn nfs_init_nfspagecache() -> int __init;
}
extern "C" {
    pub fn nfs_destroy_nfspagecache();
}
extern "C" {
    pub fn nfs_init_readpagecache() -> int __init;
}
extern "C" {
    pub fn nfs_destroy_readpagecache();
}
extern "C" {
    pub fn nfs_init_writepagecache() -> int __init;
}
extern "C" {
    pub fn nfs_destroy_writepagecache();
}
extern "C" {
    pub fn nfs_init_directcache() -> int __init;
}
extern "C" {
    pub fn nfs_destroy_directcache();
}
extern "C" {
    pub fn nfs_set_pgio_error(hdr: *mut nfs_pgio_header, error: c_int, pos: loff_t);
}
extern "C" {
    pub fn nfs_iocounter_wait(l_ctx: *mut nfs_lock_context) -> c_int;
}
extern "C" {
    pub fn nfs_pgio_header_free(: *mut nfs_pgio_header);
}
extern "C" {
    pub fn nfs_generic_pgio(: *mut nfs_pageio_descriptor, : *mut nfs_pgio_header) -> c_int;
}
extern "C" {
    pub fn nfs_free_request(req: *mut nfs_page);
}
// nfs2xdr.c
// nfs3xdr.c
// nfs4xdr.c

// nfs4proc.c

// proc.c
extern "C" {
    pub fn nfs_close_context(ctx: *mut nfs_open_context, is_sync: c_int);
}
// dir.c
extern "C" {
    pub fn nfs_readdir_record_entry_cache_hit(dir: *mut inode);
}
extern "C" {
    pub fn nfs_readdir_record_entry_cache_miss(dir: *mut inode);
}
extern "C" {
    pub fn nfs_d_prune_case_insensitive_aliases(inode: *mut inode);
}
extern "C" {
    pub fn nfs_rmdir(: *mut inode, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn nfs_unlink(: *mut inode, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn nfs_link(: *mut dentry, : *mut inode, : *mut dentry) -> c_int;
}

// file.c
extern "C" {
    pub fn nfs_file_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
}
extern "C" {
    pub fn nfs_file_llseek(: *mut file, _arg: loff_t, _arg: c_int) -> loff_t;
}
extern "C" {
    pub fn nfs_file_read(: *mut kiocb, : *mut iov_iter) -> isize;
}
extern "C" {
    pub fn nfs_file_mmap_prepare(: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn nfs_file_write(: *mut kiocb, : *mut iov_iter) -> isize;
}
extern "C" {
    pub fn nfs_file_release(: *mut inode, : *mut file) -> c_int;
}
extern "C" {
    pub fn nfs_lock(: *mut file, _arg: c_int, : *mut file_lock) -> c_int;
}
extern "C" {
    pub fn nfs_flock(: *mut file, _arg: c_int, : *mut file_lock) -> c_int;
}
extern "C" {
    pub fn nfs_check_flags(_arg: c_int) -> c_int;
}
// inode.c
extern "C" {
    pub fn nfs_free_inode(: *mut inode);
}
extern "C" {
    pub fn nfs_write_inode(: *mut inode, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn nfs_drop_inode(: *mut inode) -> c_int;
}
extern "C" {
    pub fn nfs_clear_inode(: *mut inode);
}
extern "C" {
    pub fn nfs_evict_inode(: *mut inode);
}
extern "C" {
    pub fn nfs_zap_acl_cache(inode: *mut inode);
}
extern "C" {
    pub fn nfs_set_cache_invalid(inode: *mut inode, flags: c_ulong);
}
extern "C" {
    pub fn nfs_check_cache_invalid(: *mut inode, long: unsigned) -> bool;
}
extern "C" {
    pub fn nfs_wait_bit_killable(key: *mut wait_bit_key, mode: c_int) -> c_int;
}
extern "C" {
    pub fn nfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}

// localio.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_local_dio {
    pub mem_align: u32,
    pub offset_align: u32,
    pub middle_offset: loff_t,
    pub end_offset: loff_t,
    pub /: *mut *mut ssize_t start_len; / Length for misaligned first extent,
    pub /: *mut *mut ssize_t middle_len; / Length for DIO-aligned middle extent,
    pub /: *mut *mut ssize_t end_len; / Length for misaligned last extent,
}

extern "C" {
    pub fn nfs_local_probe_async(: *mut nfs_client);
}
extern "C" {
    pub fn nfs_local_probe_async_work(: *mut work_struct);
}
extern "C" {
    pub fn nfs_server_is_local(clp: *const nfs_client) -> bool;
}

// super.c
extern "C" {
    pub fn nfs_auth_info_match(: *const nfs_auth_info, _arg: rpc_authflavor_t) -> bool;
}
extern "C" {
    pub fn nfs_try_get_tree(: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn nfs_get_tree_common(: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn nfs_kill_super(: *mut super_block);
}
extern "C" {
    pub fn register_nfs_fs() -> int __init;
}
extern "C" {
    pub fn unregister_nfs_fs() -> void __exit;
}
extern "C" {
    pub fn nfs_sb_active(sb: *mut super_block) -> bool;
}
extern "C" {
    pub fn nfs_sb_deactive(sb: *mut super_block);
}

// io.c
extern "C" {
    pub fn nfs_start_io_read(inode: *mut inode) -> __must_check int;
}
extern "C" {
    pub fn nfs_end_io_read(inode: *mut inode);
}
extern "C" {
    pub fn nfs_start_io_write(inode: *mut inode) -> __must_check int;
}
extern "C" {
    pub fn nfs_end_io_write(inode: *mut inode);
}
extern "C" {
    pub fn nfs_start_io_direct(inode: *mut inode) -> __must_check int;
}
extern "C" {
    pub fn nfs_start_io_direct_nowait(inode: *mut inode) -> __must_check int;
}
extern "C" {
    pub fn nfs_end_io_direct(inode: *mut inode);
}
// Must be called with exclusively locked inode->i_rwsem
// namespace.c
pub const NFS_PATH_CANONICAL: c_int = 1;
extern "C" {
    pub fn nfs_submount(: *mut fs_context, : *mut nfs_server) -> c_int;
}
extern "C" {
    pub fn nfs_do_submount(: *mut fs_context) -> c_int;
}
// getroot.c
extern "C" {
    pub fn nfs_get_root(s: *mut super_block, fc: *mut fs_context) -> c_int;
}

extern "C" {
    pub fn nfs4_get_rootfh(server: *mut nfs_server, mntfh: *mut nfs_fh, _arg: bool) -> c_int;
}

// read.c
extern "C" {
    pub fn nfs_read_alloc_scratch(hdr: *mut nfs_pgio_header, size: usize) -> bool;
}
extern "C" {
    pub fn nfs_pageio_complete_read(pgio: *mut nfs_pageio_descriptor);
}
extern "C" {
    pub fn nfs_pageio_reset_read_mds(pgio: *mut nfs_pageio_descriptor);
}
// super.c
extern "C" {
    pub fn nfs_umount_begin(: *mut super_block);
}
extern "C" {
    pub fn nfs_statfs(: *mut dentry, : *mut kstatfs) -> c_int;
}
extern "C" {
    pub fn nfs_show_options(: *mut seq_file, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn nfs_show_devname(: *mut seq_file, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn nfs_show_path(: *mut seq_file, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn nfs_show_stats(: *mut seq_file, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn nfs_reconfigure(: *mut fs_context) -> c_int;
}
// write.c
extern "C" {
    pub fn nfs_pageio_reset_write_mds(pgio: *mut nfs_pageio_descriptor);
}
extern "C" {
    pub fn nfs_commit_free(p: *mut nfs_commit_data);
}
extern "C" {
    pub fn nfs_commit_prepare(task: *mut rpc_task, calldata: *mut c_void);
}
extern "C" {
    pub fn nfs_reqs_to_commit(: *mut nfs_commit_info) -> c_ulong;
}
extern "C" {
    pub fn nfs_write_need_commit(: *mut nfs_pgio_header) -> c_int;
}
extern "C" {
    pub fn nfs_writeback_update_inode(hdr: *mut nfs_pgio_header);
}
extern "C" {
    pub fn nfs_commitdata_release(data: *mut nfs_commit_data);
}
extern "C" {
    pub fn nfs_key_timeout_notify(filp: *mut file, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn nfs_ctx_key_to_expire(ctx: *mut nfs_open_context, inode: *mut inode) -> bool;
}
extern "C" {
    pub fn nfs_pageio_stop_mirroring(pgio: *mut nfs_pageio_descriptor);
}

extern "C" {
    pub fn memcmp(_arg: v1->data, _arg: v2->data, _arg: sizeof(v1->data)) -> return;
}
// For workers __GFP_NORETRY only with __GFP_IO or __GFP_FS
//
// Special version of should_remove_suid() that ignores capabilities.
//
// suid always must be killed
//
// sgid without any exec bits is just a mandatory locking mark; leave
// it alone.  If some exec bits are set, it's a real sgid; kill it.
//
// unlink.c
extern "C" {
    pub fn nfs_sillyrename(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
// direct.c
extern "C" {
    pub fn nfs_dreq_bytes_left(dreq: *mut nfs_direct_req, offset: loff_t) -> isize;
}
// nfs4proc.c
//
// Determine the device name as a string
//
extern "C" {
    pub fn nfs_path(_arg: &dummy, _arg: dentry, _arg: buffer, _arg: buflen, _arg: NFS_PATH_CANONICAL) -> return;
}
//
// Determine the actual block size (and log2 thereof)
//
// make sure blocksize is a power of two
// nrbitsp = nrbits;
//
// Calculate the number of 512byte blocks used.
//
// Compute and set NFS server blocksize
//
extern "C" {
    pub fn nfs_block_bits(_arg: bsize, _arg: nrbitsp) -> return;
}
//
// Compute and set NFS server rsize / wsize
//
extern "C" {
    pub fn nfs_block_bits(_arg: iosize, _arg: NULL) -> return;
}
//
// Determine the maximum file size for a superblock
//
// Record the request's range as unstable (an extra writeback period) and
// mark its inode as dirty.
//
// This range is really still in write-back - just that the
// writeback is happening on the server now.
//
// Determine the number of bytes of data the page contains
//
extern "C" {
    pub fn folio_size(_arg: folio) -> return;
}
//
// Convert a umode to a dirent->d_type
//
// Determine the number of pages in an array of length 'len' and
// with a base offset of 'base'
//
// Convert a struct timespec64 into a 64-bit change attribute
//
// This does approximately the same thing as timespec64_to_ns(),
// but for calculation efficiency, we multiply the seconds by
// 1024*1024*1024.
//
extern "C" {
    pub fn nfs_error_is_fatal(_arg: err) -> return;
}
//
// Select between a default port value and a user-specified port value.
// If a zero value is set, then autobind will be used.
//
// port = default_port;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_direct_req {
    pub /: *mut *mut kref kref; / release manager,
// I/O parameters
    pub /: *mut *mut *mut nfs_open_context ctx; / file open context info,
    pub /: *mut *mut *mut nfs_lock_context l_ctx; / Lock context info,
    pub /: *mut *mut *mut kiocb  iocb; / controlling i/o request,
    pub /: *mut *mut *mut inode  inode; / target file of i/o,
// completion state
    pub /: *mut *mut atomic_t io_count; / i/os we're waiting for,
    pub /: *mut *mut spinlock_t lock; / protect completion state,
    pub /: *mut *mut loff_t io_start; / Start offset for I/O,
    pub /: *mut *mut error; / any reported error,
    pub /: *mut *mut completion completion; / wait for i/o completion,
// commit state
    pub /: *mut *mut nfs_mds_commit_info mds_cinfo; / Storage for cinfo,
    pub /: *mut *mut pnfs_ds_commit_info ds_cinfo; / Storage for cinfo,
    pub work: work_struct,
    pub flags: c_int,
// for write

// for read

}
