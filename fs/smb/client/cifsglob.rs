//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifsglob.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (C) International Business Machines  Corp., 2002,2008
// Author(s): Steve French (sfrench@us.ibm.com)
// Jeremy Allison (jra@samba.org)
//

pub const SMB_PATH_MAX: c_int = 260;
pub const CIFS_PORT: c_int = 445;
pub const RFC1001_PORT: c_int = 139;
//
// The sizes of various internal tables and strings
//
pub const MAX_UID_INFO: c_int = 16;
pub const MAX_SES_INFO: c_int = 2;
pub const MAX_TCON_INFO: c_int = 4;

pub const CIFS_MIN_RCV_POOL: c_int = 4;

//
// default attribute cache timeout (jiffies)
//

//
// max sleep time before retry to server
//
pub const CIFS_MAX_SLEEP: c_int = 2000;
//
// max attribute cache timeout (jiffies) - 2^30
//

//
// Max persistent and resilient handle timeout (milliseconds).
// Windows durable max was 960000 (16 minutes)
//
pub const SMB3_MAX_HANDLE_TIMEOUT: c_int = 960000;
//
// MAX_REQ is the maximum number of requests that WE will send
// on one socket concurrently.
//
pub const CIFS_MAX_REQ: c_int = 32767;
pub const RFC1001_NAME_LEN: c_int = 15;

// maximum length of ip addr as a string (including ipv6 and sctp)
pub const SERVER_NAME_LENGTH: c_int = 80;

// echo interval in seconds
pub const SMB_ECHO_INTERVAL_MIN: c_int = 1;
pub const SMB_ECHO_INTERVAL_MAX: c_int = 600;
pub const SMB_ECHO_INTERVAL_DEFAULT: c_int = 60;
// smb multichannel query server interfaces interval in seconds
pub const SMB_INTERFACE_POLL_INTERVAL: c_int = 600;
// maximum number of PDUs in one compound
pub const MAX_COMPOUND: c_int = 10;
//
// Default number of credits to keep available for SMB3.
// This value is chosen somewhat arbitrarily. The Windows client
// defaults to 128 credits, the Windows server allows clients up to
// 512 credits (or 8K for later versions), and the NetApp server
// does not limit clients at all.  Choose a high enough default value
// such that the client shouldn't limit performance, but allow mount
// to override (until you approach 64K, where we limit credits to 65000
// to reduce possibility of seeing more server credit overflow bugs.
//
pub const SMB2_MAX_CREDITS_AVAILABLE: c_int = 32000;

//
// CIFS vfs client Status information (based on what we know.)
//
// associated with each connection
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum statusEnum {
    CifsNew = 0,
    CifsGood,
    CifsExiting,
    CifsNeedReconnect,
    CifsNeedNegotiate,
    CifsInNegotiate,
}

// associated with each smb session
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ses_status_enum {
    SES_NEW = 0,
    SES_GOOD,
    SES_EXITING,
    SES_NEED_RECON,
    SES_IN_SETUP
}

// associated with each tree connection to the server
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tid_status_enum {
    TID_NEW = 0,
    TID_GOOD,
    TID_EXITING,
    TID_NEED_RECON,
    TID_NEED_TCON,
    TID_IN_TCON,
    TID_NEED_FILES_INVALIDATE, /* currently unused */
    TID_IN_FILES_INVALIDATE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum securityEnum {
    Unspecified = 0,	/* not specified */
    NTLMv2,			/* Legacy NTLM auth with NTLMv2 hash */
    RawNTLMSSP,		/* NTLMSSP without SPNEGO, NTLMv2 hash */
    Kerberos,		/* Kerberos via SPNEGO */
    IAKerb,			/* Kerberos proxy */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum upcall_target_enum {
    UPTARGET_UNSPECIFIED, /* not specified, defaults to app */
    UPTARGET_MOUNT, /* upcall to the mount namespace */
    UPTARGET_APP, /* upcall to the application namespace which did the mount */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_reparse_type {
    CIFS_REPARSE_TYPE_NONE,
    CIFS_REPARSE_TYPE_NFS,
    CIFS_REPARSE_TYPE_WSL,
    CIFS_REPARSE_TYPE_DEFAULT = CIFS_REPARSE_TYPE_NFS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_symlink_type {
    CIFS_SYMLINK_TYPE_DEFAULT,
    CIFS_SYMLINK_TYPE_NONE,
    CIFS_SYMLINK_TYPE_NATIVE,
    CIFS_SYMLINK_TYPE_UNIX,
    CIFS_SYMLINK_TYPE_MFSYMLINKS,
    CIFS_SYMLINK_TYPE_SFU,
    CIFS_SYMLINK_TYPE_NFS,
    CIFS_SYMLINK_TYPE_WSL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct session_key {
    pub len: c_uint,
    pub response: *mut c_char,
}

// encryption related structure/fields, not specific to a sec mech
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_secmech {
    pub /: *mut *mut *mut crypto_aead enc; / smb3 encryption AEAD TFM (AES-CCM and AES-GCM),
    pub /: *mut *mut *mut crypto_aead dec; / smb3 decryption AEAD TFM (AES-CCM and AES-GCM),
}

// per smb session structure/fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntlmssp_auth {
    pub /: *mut *mut bool sesskey_per_smbsess; / whether session key is per smb session,
    pub /: *mut *mut __u32 client_flags; / sent by client in type 1 ntlmsssp exchange,
    pub /: *mut *mut __u32 server_flags; / sent by server in type 2 ntlmssp exchange,
    pub /: *mut *mut unsigned char ciphertext[CIFS_CPHTXT_SIZE]; / sent to server,
    pub /: *mut *mut char cryptkey[CIFS_CRYPTO_KEY_SIZE]; / used by ntlmssp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_cred {
    pub uid: c_int,
    pub gid: c_int,
    pub mode: c_int,
    pub cecount: c_int,
    pub osid: smb_sid,
    pub gsid: smb_sid,
    pub ntaces: *mut cifs_ntace,
    pub aces: *mut smb_ace,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_open_info_data {
    pub adjust_tz: bool,
    pub reparse_point: bool,
    pub contains_posix_file_info: bool,
    pub unknown_nlink: bool,
// ioctl response buffer
    pub buftype: c_int,
    pub iov: kvec,
    pub io: },
    pub tag: __u32,
    pub buf: *mut reparse_data_buffer,
    pub reparse: },
    pub eas: [__u8; SMB2_WSL_MAX_QUERY_EA_RESP_SIZE],
    pub eas_len: c_uint,
    pub wsl: },
    pub symlink_target: *mut c_char,
    pub posix_owner: smb_sid,
    pub posix_group: smb_sid,
    pub fi: smb2_file_all_info,
    pub posix_fi: smb311_posix_qinfo,
}

//
// Except the CIFS PDUs themselves all the
// globally interesting structs should go here
//
// A smb_rqst represents a complete request to be issued to a server. It's
// formed by a kvec array, followed by an array of pages. Page data is assumed
// to start at the beginning of the first page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_rqst {
    pub /: *mut *mut *mut kvec rq_iov; / array of kvecs,
    pub /: *mut *mut unsigned int rq_nvec; / number of kvecs in array,
    pub /: *mut *mut iov_iter rq_iter; / Data iterator,
    pub /: *mut *mut *mut folio_queue rq_buffer; / Buffer for encryption,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_version_operations {
    pub xid): c_uint,
    pub ): *mut *mut *mut bool (compare_fids)(struct cifsFileInfo , struct cifsFileInfo,
// setup request: allocate mid, sign message
    pub ): *mut smb_rqst,
// setup async request: allocate mid, sign message
    pub ): *mut smb_rqst,
// check response: verify signature, map error
    pub optype): c_int,
    pub int): *const *const *const void (set_credits)(struct TCP_Server_Info ,,
    pub int): *const *const *const *const int  (get_credits_field)(struct TCP_Server_Info ,,
    pub ): *mut *mut unsigned int (get_credits)(struct mid_q_entry,
    pub ): *mut *mut __u64 (get_next_mid)(struct TCP_Server_Info,
    pub val): c_uint,
// data offset from read response message
    pub ): *mut *mut unsigned int (read_data_offset)(char,
//
// Data length from read response message
// When in_remaining is true, the returned data length is in
// message field DataRemaining for out-of-band data read (e.g through
// Memory Registration RDMA write in SMBD).
// Otherwise, the returned data length is in message field DataLength.
//
    pub in_remaining): *mut *mut *mut unsigned int (read_data_length)(char , bool,
// map smb to linux error
    pub bool): *mut *mut *mut int (map_error)(char ,,
// find mid corresponding to the response message
    pub buf): *mut *mut *mut *mut mid_q_entry (find_mid)(TCP_Server_Info server, char,
    pub ptcp_info): *mut *mut *mut void (dump_detail)(void buf, size_t buf_len, struct TCP_Server_Info,
    pub ): *mut *mut void (clear_stats)(struct cifs_tcon,
    pub ): *mut *mut *mut void (print_stats)(struct seq_file m, struct cifs_tcon,
    pub ): *mut *mut *mut void (dump_share_caps)(struct seq_file , struct cifs_tcon,
// verify the message
    pub server): *mut TCP_Server_Info,
    pub ): *mut *mut *mut bool (is_oplock_break)(char , struct TCP_Server_Info,
    pub ): *mut *mut *mut int (handle_cancelled_mid)(struct mid_q_entry , struct TCP_Server_Info,
    pub purge_cache): *mut __u16 epoch, bool,
// process transaction2 response
    pub int): *mut *mut char ,,
// check if we need to negotiate
    pub ): *mut *mut bool (need_neg)(struct TCP_Server_Info,
// negotiate to the server
    pub server): *mut TCP_Server_Info,
// set negotiated write size
    pub ctx): *mut *mut *mut unsigned int (negotiate_wsize)(struct cifs_tcon tcon, struct smb3_fs_context,
// set negotiated read size
    pub ctx): *mut *mut *mut unsigned int (negotiate_rsize)(struct cifs_tcon tcon, struct smb3_fs_context,
// setup smb sessionn
    pub ): *const nls_table,
// close smb session
    pub ): *const *const int (logoff)(unsigned int, struct cifs_ses,
// connect to a server share
    pub ): *const *const cifs_tcon , nls_table,
// close tree connection
    pub ): *const *const int (tree_disconnect)(unsigned int, struct cifs_tcon,
// get DFS referrals
    pub int): *const *const *const unsigned int , struct nls_table ,,
// informational QFS call
    pub ): *mut cifs_sb_info,
// query for server interfaces
// check if a path is accessible or not
    pub ): *const *const cifs_sb_info , char,
// query path data from the server
    pub data): *mut cifs_open_info_data,
// query file data from the server
    pub data): *mut *mut cifsFileInfo cfile, cifs_open_info_data,
// query reparse point to determine which type of special file
    pub rsp_buftype): *mut c_int,
// get server index number
    pub data): *mut cifs_open_info_data,
// set size by path
    pub ): *mut dentry,
// set size by file handle
    pub bool): *mut *mut cifsFileInfo , __u64,,
// set attributes
    pub int): unsigned,
    pub __u16): *mut *mut cifsFileInfo ,,
// check if we can send an echo or nor
    pub ): *mut *mut bool (can_echo)(struct TCP_Server_Info,
// send echo request
    pub ): *mut *mut int (echo)(struct TCP_Server_Info,
// create directory
    pub cifs_sb): *mut cifs_sb_info,
    pub sb): *mut cifs_sb_info,
// set info on created directory
    pub int): unsigned,
// remove directory
    pub ): *mut cifs_sb_info,
// unlink file
    pub ): *mut *mut cifs_sb_info , dentry,
// open, rename and delete file
    pub int): unsigned,
// send rename request
    pub cifs_sb): *mut cifs_sb_info,
// send create hardlink request
    pub cifs_sb): *mut cifs_sb_info,
// query symlink target
    pub target_path): *mut c_char,
// open a file for non-posix mounts
    pub buf): *mut c_void,
// set fid protocol-specific info
    pub __u32): *mut *mut *mut *mut void (set_fid)(struct cifsFileInfo , struct cifs_fid ,,
// close a file
    pub ): *mut cifs_fid,
// close a file, returning file attributes and timestamps
    pub pfile_info): *mut cifsFileInfo,
// send a flush request to the server
    pub ): *const *const *const int (flush)(unsigned int, struct cifs_tcon , struct cifs_fid,
// async read from the server
    pub ): *mut *mut int (async_readv)(struct cifs_io_subrequest,
// async write to the server
    pub ): *mut *mut void (async_writev)(struct cifs_io_subrequest,
// sync read from the server
    pub ): *mut c_int,
// sync write to the server
    pub long): unsigned,
// open dir, start readdir
    pub ): *mut cifs_search_info,
// continue readdir
    pub srch_inf): *mut __u16, struct cifs_search_info,
// close dir
    pub ): *mut cifs_fid,
// calculate a size of SMB message
    pub buf): *mut *mut unsigned int (calc_smb_size)(void,
// check for STATUS_PENDING and process the response if yes
    pub server): *mut *mut *mut bool (is_status_pending)(char buf, struct TCP_Server_Info,
// check for STATUS_NETWORK_SESSION_EXPIRED
    pub ): *mut *mut bool (is_session_expired)(char,
// send oplock break response
    pub oplock): c_uint,
// query remote filesystem
    pub ): *const *const *const char , struct cifs_sb_info , struct kstatfs,
// send mandatory brlock to the server
    pub bool): __u64, __u32, int, int,,
// unlock range of mandatory locks
    pub int): unsigned,
// push brlocks from the cache to the server
    pub ): *mut *mut int (push_mand_locks)(struct cifsFileInfo,
// get lease key of the inode
    pub ): *mut *mut *mut void (get_lease_key)(struct inode , struct cifs_fid,
// set lease key of the inode
    pub ): *mut *mut *mut void (set_lease_key)(struct inode , struct cifs_fid,
// generate new lease key
    pub ): *mut *mut void (new_lease_key)(struct cifs_fid,
    pub server): *mut TCP_Server_Info,
    pub src_file): *mut cifsFileInfo,
    pub ): *mut *mut cifsFileInfo src_file, void __user,
    pub return_changes): *mut *mut void __user pbuf, bool,
    pub ): *mut *mut char , unsigned int,
    pub ): *mut *mut char , unsigned int,
// if we can do cache read operations
    pub (*is_read_op)(__u32): *mut bool,
// set oplock level for the inode
    pub purge_cache): *mut bool,
// create lease context buffer for CREATE request
    pub le_flags): *mut *mut *mut *mut *mut char  (create_lease_buf)(u8 lease_key, u8 oplock, u8 parent_lease_key, __le32,
// parse lease context buffer and return oplock/epoch info
    pub lkey): *mut *mut *mut *mut __u8 (parse_lease_buf)(void buf, __u16 epoch, char,
    pub dest_off): u64 src_off, u64 len, u64,
    pub dest_off): u64,
    pub ): *const *const int (validate_negotiate)(unsigned int, struct cifs_tcon,
    pub ): *mut size_t, struct cifs_sb_info,
    pub ): *const *const nls_table , cifs_sb_info,
    pub info): *const *const *const char patch, u32 plen, u32,
    pub info): *const *const *const cifs_fid pfid, u32 plen, u32,
    pub flag): c_int,
// writepages retry size
    pub ): *mut *mut unsigned int (wp_retry_size)(struct inode,
// get mtu credits
    pub ): *mut *mut size_t , struct cifs_credits,
// adjust previously taken mtu credits to request size
    pub trace): *mut *mut *mut unsigned int /enum smb3_rw_credits_trace/,
// check if we need to issue closedir
    pub ): *mut *mut bool (dir_needs_close)(struct cifsFileInfo,
// init transform (compress/encrypt) request
    pub ): *mut *mut smb_rqst , smb_rqst,
    pub buf): *mut *mut int (is_transform_hdr)(void,
    pub ): *mut *mut *mut *mut *mut mid_q_entry , char , int,
    pub securityEnum): enum,
    pub noff): *mut c_uint,
// ioctl passthrough for query_info
    pub p): c_ulong,
// make unix special files (block, char, fifo, socket)
    pub device_number): dev_t,
// version specific fiemap implementation
    pub u64): *mut *mut fiemap_extent_info , u64,,
// version specific llseek implementation
    pub int): *mut *mut *mut *mut loff_t (llseek)(struct file , struct cifs_tcon , loff_t,,
// Check for STATUS_IO_TIMEOUT
    pub buf): *mut *mut bool (is_status_io_timeout)(char,
// Check for STATUS_NETWORK_NAME_DELETED
    pub srv): *mut *mut *mut bool (is_network_name_deleted)(char buf, struct TCP_Server_Info,
    pub plen): *mut u32,
    pub xattr_iov): *mut kvec,
}

//
// CIFS superblock mount flags (mnt_cifs_flags) to consider when
// trying to reuse existing superblock for a new mount
//

//
// Generic VFS superblock mount flags (s_flags) to consider when
// trying to reuse existing superblock for a new mount
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_mnt_data {
    pub cifs_sb: *mut cifs_sb_info,
    pub ctx: *mut smb3_fs_context,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCP_Server_Info {
    pub tcp_ses_list: list_head,
    pub smb_ses_list: list_head,
    pub /: *mut *mut list_head rlist; / reconnect list,
    pub /: *mut *mut spinlock_t srv_lock; / protect anything here that is not protected,
    pub /: *mut *mut __u64 conn_id; / connection identifier (useful for debugging),
    pub /: *mut *mut int srv_count; / reference counter,
    pub /: *mut *mut int rfc1001_sessinit; / whether to estasblish netbios session,
    pub /: *mut *mut bool with_rfc1001; / if netbios session is used,
// 15 character server name + 0x20 16th byte indicating type = srv
    pub server_RFC1001_name: [c_char; RFC1001_NAME_LEN_WITH_NULL],
    pub ops: *mut smb_version_operations,
    pub vals: *mut smb_version_values,
// updates to tcpStatus protected by cifs_tcp_ses_lock
    pub /: *mut *mut statusEnum tcpStatus; / what we think the status is,
    pub /: *mut *mut *mut char hostname; / hostname portion of UNC string,
    pub ssocket: *mut socket,
    pub dstaddr: sockaddr_storage,
    pub /: *mut *mut sockaddr_storage srcaddr; / locally bind to this IP,

    pub net: *mut net,

    pub response_q: wait_queue_head_t,
    pub block*/: *mut *mut wait_queue_head_t request_q; / if more than maxmpx to srvr must,
    pub /: *mut *mut spinlock_t mid_queue_lock; / protect mid queue,
    pub mid_counter_lock: spinlock_t,
    pub pending_mid_q: list_head,
    pub /: *mut *mut bool noblocksnd; / use blocking sendmsg,
    pub /: *mut *mut bool noautotune; / do not autotune send buf sizes,
    pub nosharesock: bool,
    pub tcp_nodelay: bool,
    pub terminate: bool,
    pub /: *mut *mut int credits; / send no more requests at once,
    pub /: *mut *mut unsigned int max_credits; / can override large 32000 default at mnt,
    pub /: *mut *mut unsigned int in_flight; / number of requests on the wire to server,
    pub /: *mut *mut unsigned int max_in_flight; / max number of requests that were on wire,
    pub /: *mut *mut spinlock_t req_lock; / protect the two values above,
    pub _srv_mutex: mutex,
    pub nofs_flag: c_uint,
    pub tsk: *mut task_struct,
    pub server_GUID: [c_char; 16],
    pub sec_mode: __u16,
    pub /: *mut *mut bool sign; / is signing enabled on this connection?,
    pub /: *mut *mut bool ignore_signature:1; / skip validation of signatures in SMB2/3 rsp,
    pub /: *mut *mut bool session_estab; / mark when very first sess is established,
    pub /: *mut *mut int echo_credits; / echo reserved slots,
    pub /: *mut *mut int oplock_credits; / oplock break reserved slots,
    pub /: *mut *mut bool echoes:1; / enable echoes,
    pub /: *mut *mut __u8 client_guid[SMB2_CLIENT_GUID_SIZE]; / Client GUID,
    pub /: *mut *mut u16 dialect; / dialect index that server chose,
    pub /: *mut *mut bool oplocks:1; / enable oplocks,
    pub /: *mut *mut unsigned int maxReq; / Clients should submit no more,
// than maxReq distinct unanswered SMBs to the server when using
// multiplexed reads or writes (for SMB1/CIFS only, not SMB2/SMB3)
    pub /: *mut *mut unsigned int maxBuf; / maxBuf specifies the maximum,
// message size the server can send or receive for non-raw SMBs
// maxBuf is returned by SMB NegotiateProtocol so maxBuf is only 0
// when socket is setup (and during reconnect) before NegProt sent
    pub /: *mut *mut unsigned int max_rw; / maxRw specifies the maximum,
// message size the server can send or receive for
// SMB_COM_WRITE_RAW or SMB_COM_READ_RAW.
    pub /: *mut *mut unsigned int capabilities; / selective disabling of caps by smb sess,
    pub /: *mut *mut int timeAdj; / Adjust for difference in server time zone in sec,
    pub /: *mut *mut __u64 current_mid; / multiplex id - rotating counter, protected by mid_counter_lock,
    pub /: *mut *mut char cryptkey[CIFS_CRYPTO_KEY_SIZE]; / used by ntlm, ntlmv2 etc,
// 16th byte of RFC1001 workstation name is always null
    pub workstation_RFC1001_name: [c_char; RFC1001_NAME_LEN_WITH_NULL],
    pub /: *mut *mut __u32 sequence_number; / for signing, protected by srv_mutex,
    pub /: *mut *mut __u32 reconnect_instance; / incremented on each reconnect,
    pub /: *mut *mut __le32 session_key_id; / retrieved from negotiate response and send in session setup request,
    pub session_key: session_key,
    pub /: *mut *mut unsigned long lstrp; / when we got last response from this server,
    pub /: *mut *mut unsigned long neg_start; / when negotiate started (jiffies),
    pub /: *mut *mut unsigned long reconn_delay; / when resched session and tcon reconnect,
    pub /: *mut *mut cifs_secmech secmech; / crypto sec mech functs, descriptors,

    pub /: *mut *mut char negflavor; / NEGOTIATE response flavor,
// extended security flavors that server supports
    pub /: *mut *mut bool sec_ntlmssp; / supports NTLMSSP,
    pub /: *mut *mut bool sec_kerberosu2u; / supports U2U Kerberos,
    pub /: *mut *mut bool sec_kerberos; / supports plain Kerberos,
    pub /: *mut *mut bool sec_mskerberos; / supports legacy MS Kerberos,
    pub /: *mut *mut bool sec_iakerb; / supports pass-through auth for Kerberos (krb5 proxy),
    pub /: *mut *mut bool large_buf; / is current buffer large?,
// use SMBD connection instead of socket
    pub rdma: bool,
// point to the SMBD connection if RDMA is used instead of socket
    pub smbd_conn: *mut smbd_connection,
    pub /: *mut *mut delayed_work echo; / echo ping workqueue job,
    pub /: *mut *mut *mut char smallbuf; / pointer to current "small" buffer,
    pub /: *mut *mut *mut char bigbuf; / pointer to current "big" buffer,
// Total size of this PDU. Only valid from cifs_demultiplex_thread
    pub pdu_size: c_uint,
    pub /: *mut *mut unsigned int total_read; / total amount of data read in this pass,
    pub /: *mut *mut atomic_t in_send; / requests trying to send,
    pub /: *mut *mut atomic_t num_waiters; / blocked waiting to get in sendrecv,

    pub /: *mut *mut atomic_t num_cmds[NUMBER_OF_SMB2_COMMANDS]; / total requests by cmd,
    pub /: *mut *mut atomic_t smb2slowcmd[NUMBER_OF_SMB2_COMMANDS]; / count resps > 1 sec,
    pub /: *mut *mut __u64 time_per_cmd[NUMBER_OF_SMB2_COMMANDS]; / total time per cmd,
    pub slowest_cmd: [__u32; NUMBER_OF_SMB2_COMMANDS],
    pub fastest_cmd: [__u32; NUMBER_OF_SMB2_COMMANDS],
    pub max_read: c_uint,
    pub max_write: c_uint,
    pub min_offload: c_uint,
//
// If payload is less than or equal to the threshold,
// use RDMA send/recv to send upper layer I/O.
// If payload is more than the threshold,
// use RDMA read/write through memory registration for I/O.
//
    pub rdma_readwrite_threshold: c_uint,
    pub retrans: c_uint,
    pub set*/: *mut *mut bool requested; / "compress" mount option,
    pub /: *mut *mut bool enabled; / actually negotiated with server,
    pub /: *mut *mut bool chained; / chained transforms were negotiated,
    pub /: *mut *mut bool pattern; / Pattern_V1 chained payloads were negotiated,
    pub /: *mut *mut __le16 alg; / preferred alg negotiated with server,
    pub compression: },
    pub signing_algorithm: __u16,
    pub cipher_type: __le16,
// save initial negprot hash
    pub preauth_sha_hash: [__u8; SMB2_PREAUTH_HASH_SIZE],
    pub /: *mut *mut bool signing_negotiated; / true if valid signing context rcvd from server,
    pub posix_ext_supported: bool,
    pub /: *mut *mut delayed_work reconnect; / reconnect workqueue job,
    pub /: *mut *mut mutex reconnect_mutex; / prevent simultaneous reconnects,
    pub echo_interval: c_ulong,
//
// Number of targets available for reconnect. The more targets
// the more tasks have to wait to let the demultiplex thread
// reconnect.
//
    pub nr_targets: c_int,
    pub /: *mut *mut bool noblockcnt; / use non-blocking connect(),
//
// If this is a session channel,
// primary_server holds the ref-counted
// pointer to primary channel connection for the session.
//

    pub primary_server: *mut TCP_Server_Info,
    pub /: *mut *mut __u16 channel_sequence_num; / incremented on primary channel on each chan reconnect,

    pub use_swn_dstaddr: bool,
    pub swn_dstaddr: sockaddr_storage,

//
// Canonical DFS referral path used in cifs_reconnect() for failover as
// well as in DFS cache refresher.
//
// format: \\HOST\SHARE[\OPTIONAL PATH]
//
    pub leaf_fullpath: *mut c_char,
    pub dfs_conn:1: bool,
    pub 1]: char dns_dom[CIFS_MAX_DOMAINNAME_LEN +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_credits {
    pub value: c_uint,
    pub instance: c_uint,
    pub in_flight_check: c_uint,
    pub rreq_debug_id: c_uint,
    pub rreq_debug_index: c_uint,
}

extern "C" {
    pub fn cpu_to_le64(_arg: server->ops->get_next_mid(server)) -> return;
}
//
// The value in the SMB header should be little endian for easy
// on-the-wire decoding.
//
extern "C" {
    pub fn cpu_to_le16(_arg: mid) -> return;
}
extern "C" {
    pub fn revert_current_mid(_arg: server, 1: num > 0 ? num :) -> return;
}
//
// When the server supports very large reads and writes via POSIX extensions,
// we can allow up to 2^24-1, minus the size of a READ/WRITE_AND_X header, not
// including the RFC1001 length.
//
// Note that this might make for "interesting" allocation problems during
// writeback however as we have to allocate an array of pointers for the
// pages. A 16M write means ~32kb page array with PAGE_SIZE == 4096.
//
// For reads, there is a similar problem as we need to allocate an array
// of kvecs to handle the receive, though that should only need to be done
// once.
//

//
// When the server doesn't allow large posix writes, only allow a rsize/wsize
// of 2^17-1 minus the size of the call header. That allows for a read or
// write up to the maximum size described by RFC1002.
//

//
// Windows only supports a max of 60kb reads and 65535 byte writes. Default to
// those values when posix extensions aren't in force. In actuality here, we
// use 65536 to allow for a write that is a multiple of 4k. Most servers seem
// to be ok with the extra byte even though Windows doesn't send writes that
// are that large.
//
// Citation:
//
// https://blogs.msdn.com/b/openspecification/archive/2009/04/10/smb-maximum-transmit-buffer-size-and-performance-tuning.aspx
//

//
// Macros to allow the TCP_Server_Info->net field and related code to drop out
// when CONFIG_NET_NS isn't set.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_server_iface {
    pub iface_head: list_head,
    pub refcount: kref,
    pub speed: usize,
    pub weight_fulfilled: usize,
    pub num_channels: c_uint,
    pub 1: unsigned int rdma_capable :,
    pub 1: unsigned int rss_capable :,
    pub /: *mut *mut unsigned int is_active : 1; / unset if non existent,
    pub sockaddr: sockaddr_storage,
}

// release iface when last ref is dropped
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_chan {
    pub /: *mut *mut unsigned int in_reconnect : 1; / if session setup in progress for this channel,
    pub server: *mut TCP_Server_Info,
    pub /: *mut *mut *mut cifs_server_iface iface; / interface in use,
    pub signkey: [__u8; SMB3_SIGN_KEY_SIZE],
}

//
// Session structure.  One of these for each uid session with a particular host
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_ses {
    pub smb_ses_list: list_head,
    pub /: *mut *mut list_head rlist; / reconnect list,
    pub tcon_list: list_head,
    pub /: *mut *mut list_head dlist; / dfs list,
    pub tcon_ipc: *mut cifs_tcon,
    pub /: *mut *mut spinlock_t ses_lock; / protect anything here that is not protected,
    pub session_mutex: mutex,
    pub /: *mut *mut *mut TCP_Server_Info server; / pointer to server info,
    pub /: *mut *mut int ses_count; / reference counter,
    pub /: *mut *mut ses_status_ses_status; / updates protected by cifs_tcp_ses_lock,
    pub /: *mut *mut unsigned int overrideSecFlg; / if non-zero override global sec flags,
    pub /: *mut *mut *mut char serverOS; / name of operating system underlying server,
    pub /: *mut *mut *mut char serverNOS; / name of network operating system of server,
    pub /: *mut *mut *mut char serverDomain; / security realm of server,
    pub /: *mut *mut __u64 Suid; / remote smb uid,
    pub /: *mut *mut kuid_t linux_uid; / overriding owner of files on the mount,
    pub /: *mut *mut kuid_t cred_uid; / owner of credentials,
    pub capabilities: c_uint,
    pub /: *mut *mut char ip_addr[INET6_ADDRSTRLEN + 1]; / Max ipv6 (or v4) addr string len,
    pub sess: *mut *mut *mut char user_name; / must not be null except during init of,
    pub domainName: *mut c_char,
    pub password: *mut c_char,
    pub /: *mut *mut *mut char password2; / When key rotation used, new password may be set before it expires,
    pub workstation_name: [c_char; CIFS_MAX_WORKSTATION_LEN],
    pub auth_key: session_key,
    pub /: *mut *mut *mut ntlmssp_auth ntlmssp; / ciphertext, flags, server challenge,
    pub /: *mut *mut securityEnum sectype; / what security flavor was specified?,
    pub /: *mut *mut upcall_target_upcall_target; / what upcall target was specified?,
    pub /: *mut *mut bool sign; / is signing required?,
    pub domainAuto:1: bool,
    pub /: *mut *mut bool expired_pwd; / track if access denied or expired pwd so can know if need to update,
    pub unicode: c_int,
    pub flags: c_uint,
    pub session_flags: __u16,
    pub smb3signingkey: [__u8; SMB3_SIGN_KEY_SIZE],
    pub smb3encryptionkey: [__u8; SMB3_ENC_DEC_KEY_SIZE],
    pub smb3decryptionkey: [__u8; SMB3_ENC_DEC_KEY_SIZE],
    pub preauth_sha_hash: [__u8; SMB2_PREAUTH_HASH_SIZE],
//
// Network interfaces available on the server this session is
// connected to.
//
// Other channels can be opened by connecting and binding this
// session to interfaces from this list.
//
// iface_lock should be taken when accessing any of these fields
//
    pub iface_lock: spinlock_t,
// ========= begin: protected by iface_lock ========
    pub iface_list: list_head,
    pub iface_count: usize,
    pub /: *mut *mut unsigned long iface_last_update; / jiffies,
// ========= end: protected by iface_lock ========
    pub chan_lock: spinlock_t,
// ========= begin: protected by chan_lock ========
pub const CIFS_MAX_CHANNELS: c_int = 16;
    pub chans: [cifs_chan; CIFS_MAX_CHANNELS],
    pub chan_count: usize,
    pub chan_max: usize,
    pub /: *mut *mut atomic_t chan_seq; / round robin state,
//
// chans_need_reconnect is a bitmap indicating which of the channels
// under this smb session needs to be reconnected.
// If not multichannel session, only one bit will be used.
//
// We will ask for sess and tcon reconnection only if all the
// channels are marked for needing reconnection. This will
// enable the sessions on top to continue to live till any
// of the channels below are active.
//
    pub chans_need_reconnect: c_ulong,
// ========= end: protected by chan_lock ========
    pub dfs_root_ses: *mut cifs_ses,
    pub local_nls: *mut nls_table,
    pub /: *mut *mut *mut char dns_dom; / FQDN of the domain,
}

//
// common struct for holding inode info when searching for or updating an
// inode with new info
//
pub const CIFS_FATTR_JUNCTION: c_uint = 0x1;
pub const CIFS_FATTR_DELETE_PENDING: c_uint = 0x2;
pub const CIFS_FATTR_NEED_REVAL: c_uint = 0x4;
pub const CIFS_FATTR_INO_COLLISION: c_uint = 0x8;
pub const CIFS_FATTR_UNKNOWN_NLINK: c_uint = 0x10;
pub const CIFS_FATTR_FAKE_ROOT_INO: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_fattr {
    pub cf_flags: u32,
    pub cf_cifsattrs: u32,
    pub cf_uniqueid: u64,
    pub cf_eof: u64,
    pub cf_bytes: u64,
    pub cf_createtime: u64,
    pub cf_uid: kuid_t,
    pub cf_gid: kgid_t,
    pub cf_mode: umode_t,
    pub cf_rdev: dev_t,
    pub cf_nlink: c_uint,
    pub cf_dtype: c_uint,
    pub cf_atime: timespec64,
    pub cf_mtime: timespec64,
    pub cf_ctime: timespec64,
    pub cf_cifstag: u32,
    pub cf_symlink_target: *mut c_char,
}

//
// there is one of these for each connection to a resource on a particular
// session
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_tcon {
    pub tcon_list: list_head,
    pub /: *mut *mut int debug_id; / Debugging for tracing,
    pub tc_count: c_int,
    pub /: *mut *mut list_head rlist; / reconnect list,
    pub /: *mut *mut spinlock_t tc_lock; / protect anything here that is not protected,
    pub /: *mut *mut atomic_t num_local_opens; / num of all opens including disconnected,
    pub /: *mut *mut atomic_t num_remote_opens; / num of all network opens on server,
    pub openFileList: list_head,
    pub /: *mut *mut spinlock_t open_file_lock; / protects list above,
    pub /: *mut *mut *mut cifs_ses ses; / pointer to session associated with,
    pub /: *mut *mut char tree_name[MAX_TREE_SIZE + 1]; / UNC name of resource in ASCII,
    pub nativeFileSystem: *mut c_char,
    pub /: *mut *mut *mut char password; / for share-level security,
    pub /: *mut *mut __u32 tid; / The 4 byte tree id,
    pub /: *mut *mut __u16 Flags; / optional support bits,
    pub status: tid_status_enum,
    pub num_smbs_sent: core::sync::atomic::AtomicI32,
    pub num_writes: core::sync::atomic::AtomicI32,
    pub num_reads: core::sync::atomic::AtomicI32,
    pub num_flushes: core::sync::atomic::AtomicI32,
    pub num_oplock_brks: core::sync::atomic::AtomicI32,
    pub num_opens: core::sync::atomic::AtomicI32,
    pub num_closes: core::sync::atomic::AtomicI32,
    pub num_deletes: core::sync::atomic::AtomicI32,
    pub num_mkdirs: core::sync::atomic::AtomicI32,
    pub num_posixopens: core::sync::atomic::AtomicI32,
    pub num_posixmkdirs: core::sync::atomic::AtomicI32,
    pub num_rmdirs: core::sync::atomic::AtomicI32,
    pub num_renames: core::sync::atomic::AtomicI32,
    pub num_t2renames: core::sync::atomic::AtomicI32,
    pub num_ffirst: core::sync::atomic::AtomicI32,
    pub num_fnext: core::sync::atomic::AtomicI32,
    pub num_fclose: core::sync::atomic::AtomicI32,
    pub num_hardlinks: core::sync::atomic::AtomicI32,
    pub num_symlinks: core::sync::atomic::AtomicI32,
    pub num_locks: core::sync::atomic::AtomicI32,
    pub num_acl_get: core::sync::atomic::AtomicI32,
    pub num_acl_set: core::sync::atomic::AtomicI32,
    pub cifs_stats: },
    pub smb2_com_sent: [core::sync::atomic::AtomicI32; NUMBER_OF_SMB2_COMMANDS],
    pub smb2_com_failed: [core::sync::atomic::AtomicI32; NUMBER_OF_SMB2_COMMANDS],
    pub smb2_stats: },
    pub stats: },
    pub bytes_read: __u64,
    pub bytes_written: __u64,
    pub /: *mut *mut spinlock_t stat_lock; / protects the two fields above,
    pub stats_from_time: time64_t,
    pub fsDevInfo: FILE_SYSTEM_DEVICE_INFO,
    pub /: *mut *mut FILE_SYSTEM_ATTRIBUTE_INFO fsAttrInfo; / ok if fs name truncated,
    pub fsUnixInfo: FILE_SYSTEM_UNIX_INFO,
    pub /: *mut *mut bool ipc:1; / set if connection to IPC$ share (always also pipe),
    pub /: *mut *mut bool pipe:1; / set if connection to pipe share,
    pub /: *mut *mut bool print:1; / set if connection to printer share,
    pub retry:1: bool,
    pub nocase:1: bool,
    pub /: *mut *mut bool nohandlecache:1; / if strange server resource prob can turn off,
    pub nodelete:1: bool,
    pub /: *mut *mut bool seal:1; / transport encryption for this mounted share,
    pub protocol: *mut *mut bool unix_ext:1; / if false disable Linux extensions to CIFS,
    pub /: *mut *mut bool posix_extensions; / if true SMB3.11 posix extensions enabled,
    pub /: *mut *mut bool local_lease:1; / check leases (only) on local system not remote,
    pub /: *mut *mut bool broken_posix_open; / e.g. Samba server versions < 3.3.2, 3.2.9,
    pub /: *mut *mut bool broken_sparse_sup; / if server or share does not support sparse,
    pub /: *mut *mut bool need_reconnect:1; / connection reset, tid now invalid,
    pub /: *mut *mut bool need_reopen_files:1; / need to reopen tcon file handles,
    pub /: *mut *mut bool use_resilient:1; / use resilient instead of durable handles,
    pub /: *mut *mut bool use_persistent:1; / use persistent instead of durable handles,
    pub /: *mut *mut bool no_lease:1; / Do not request leases on files or directories,
    pub /: *mut *mut bool use_witness:1; / use witness protocol,
    pub /: *mut *mut bool dummy:1; / dummy tcon used for reconnecting channels,
    pub capabilities: __le32,
    pub share_flags: __u32,
    pub maximal_access: __u32,
    pub vol_serial_number: __u32,
    pub vol_create_time: __le64,
    pub /: *mut *mut __u64 snapshot_time; / for timewarp tokens - timestamp of snapshot,
    pub /: *mut *mut __u32 handle_timeout; / persistent and durable handle timeout in ms,
    pub /: *mut *mut __u32 ss_flags; / sector size flags,
    pub /: *mut *mut __u32 perf_sector_size; / best sector size for perf,
    pub max_chunks: __u32,
    pub max_bytes_chunk: __u32,
    pub max_bytes_copy: __u32,
    pub max_cached_dirs: __u32,

    pub /: *mut *mut u64 resource_id; / server resource id,
    pub /: *mut *mut bool fscache_acquired; / T if we've tried acquiring a cookie,
    pub /: *mut *mut *mut fscache_volume fscache; / cookie for share,
    pub /: *mut *mut mutex fscache_lock; / Prevent regetting a cookie,

    pub /: *mut *mut list_head pending_opens; / list of incomplete opens,
    pub cfids: *mut cached_fids,
    pub cifs_sb_list: list_head,
    pub sb_list_lock: spinlock_t,

    pub dfs_cache_work: delayed_work,
    pub dfs_ses_list: list_head,

    pub /: *mut *mut delayed_work query_interfaces; / query interfaces workqueue job,
    pub /: *mut *mut *mut char origin_fullpath; / canonical copy of smb3_fs_context::source,
}

//
// This is a refcounted and timestamped container for a tcon pointer. The
// container holds a tcon reference. It is considered safe to free one of
// these when the tl_count goes to 0. The tl_time is the time of the last
// "get" on the container.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcon_link {
    pub tl_rbnode: rb_node,
    pub tl_uid: kuid_t,
    pub tl_flags: c_ulong,
pub const TCON_LINK_MASTER: c_int = 0;
pub const TCON_LINK_PENDING: c_int = 1;
pub const TCON_LINK_IN_TREE: c_int = 2;
    pub tl_time: c_ulong,
    pub tl_count: core::sync::atomic::AtomicI32,
    pub tl_tcon: *mut cifs_tcon,
}

extern "C" {
    pub fn smb3_free_compound_rqst(num_rqst: c_int, rqst: *mut smb_rqst);
}
extern "C" {
    pub fn cifs_put_tlink(tlink: *mut tcon_link);
}
// This function is always expected to succeed
pub const CIFS_OPLOCK_NO_CHANGE: c_uint = 0xfe;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_pending_open {
    pub olist: list_head,
    pub tlink: *mut tcon_link,
    pub lease_key: [__u8; 16],
    pub oplock: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_deferred_close {
    pub dlist: list_head,
    pub tlink: *mut tcon_link,
    pub netfid: __u16,
    pub persistent_fid: __u64,
    pub volatile_fid: __u64,
}

//
// This info hangs off the cifsFileInfo structure, pointed to by llist.
// This is used to track byte stream locks on the file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifsLockInfo {
    pub /: *mut *mut list_head llist; / pointer to next cifsLockInfo,
    pub /: *mut *mut list_head blist; / pointer to locks blocked on this,
    pub block_q: wait_queue_head_t,
    pub offset: __u64,
    pub length: __u64,
    pub pid: __u32,
    pub type: __u16,
    pub flags: __u16,
}

//
// One of these for each open instance of a file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_search_info {
    pub index_of_last_entry: loff_t,
    pub entries_in_buffer: __u16,
    pub info_level: __u16,
    pub resume_key: __u32,
    pub ntwrk_buf_start: *mut c_char,
    pub srch_entries_start: *mut c_char,
    pub last_entry: *mut c_char,
    pub presume_name: *const c_char,
    pub resume_name_len: c_uint,
    pub endOfSearch:1: bool,
    pub emptyDir:1: bool,
    pub unicode:1: bool,
    pub /: *mut *mut bool smallBuf:1; / so we know which buf_release function to call,
    pub /: *mut *mut bool is_dynamic_buf:1; / dynamically allocated buffer - can be variable size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_open_parms {
    pub tcon: *mut cifs_tcon,
    pub cifs_sb: *mut cifs_sb_info,
    pub disposition: c_int,
    pub desired_access: c_int,
    pub create_options: c_int,
    pub path: *const c_char,
    pub fid: *mut cifs_fid,
    pub mode: umode_t,
    pub reconnect:1: bool,
    pub /: *mut *mut bool replay:1; / indicates that this open is for a replay,
    pub ea_cctx: *mut kvec,
    pub lease_flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_fid {
    pub netfid: __u16,
    pub /: *mut *mut __u64 persistent_fid; / persist file id for smb2,
    pub /: *mut *mut __u64 volatile_fid; / volatile file id for smb2,
    pub /: *mut *mut __u8 lease_key[SMB2_LEASE_KEY_SIZE]; / lease key for smb2,
    pub parent_lease_key: [__u8; SMB2_LEASE_KEY_SIZE],
    pub create_guid: [__u8; 16],
    pub access: __u32,
    pub pending_open: *mut cifs_pending_open,
    pub epoch: __u16,

    pub mid: __u64,

    pub purge_cache: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_fid_locks {
    pub llist: list_head,
    pub /: *mut *mut *mut cifsFileInfo cfile; / fid that owns locks,
    pub /: *mut *mut list_head locks; / locks held by fid above,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifsFileInfo {
// following two lists are protected by tcon->open_file_lock
    pub /: *mut *mut list_head tlist; / pointer to next fid owned by tcon,
    pub /: *mut *mut list_head flist; / next fid (file instance) for this inode,
// lock list below protected by cifsi->lock_sem
    pub /: *mut *mut *mut cifs_fid_locks llist; / brlocks held by this fid,
    pub /: *mut *mut kuid_t uid; / allows finding which FileInfo structure,
    pub /: *mut *mut __u32 pid; / process id who opened file,
    pub /: *mut *mut cifs_fid fid; / file id from remote,
    pub /: *mut *mut list_head rlist; / reconnect list,
// BB add lock scope info here if needed
// lock scope id (0 if none)
    pub dentry: *mut dentry,
    pub tlink: *mut tcon_link,
    pub f_flags: c_uint,
    pub /: *mut *mut bool invalidHandle:1; / file closed via session abend,
    pub swapfile:1: bool,
    pub oplock_break_cancelled:1: bool,
    pub /: *mut *mut bool status_file_deleted:1; / file has been deleted,
    pub /: *mut *mut bool offload:1; / offload final part of _put to a wq,
    pub /: *mut *mut __u16 oplock_epoch; / epoch from the lease break,
    pub /: *mut *mut __u32 oplock_level; / oplock/lease level from the lease break,
    pub count: c_int,
    pub /: *mut *mut spinlock_t file_info_lock; / protects four flag/count fields above,
    pub ses*/: *mut *mut mutex fh_mutex; / prevents reopen race after dead,
    pub srch_inf: cifs_search_info,
    pub /: *mut *mut work_oplock_break; / work for oplock breaks,
    pub /: *mut *mut work_put; / work for the final part of _put,
    pub /: *mut *mut work_serverclose; / work for serverclose,
    pub deferred: delayed_work,
    pub /: *mut *mut bool deferred_close_scheduled; / Flag to indicate close is scheduled,
    pub symlink_target: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_io_parms {
    pub netfid: __u16,
    pub /: *mut *mut __u64 persistent_fid; / persist file id for smb2,
    pub /: *mut *mut __u64 volatile_fid; / volatile file id for smb2,
    pub pid: __u32,
    pub offset: __u64,
    pub length: c_uint,
    pub tcon: *mut cifs_tcon,
    pub server: *mut TCP_Server_Info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_io_request {
    pub rreq: netfs_io_request,
    pub cfile: *mut cifsFileInfo,
    pub pid: pid_t,
}

// asynchronous read support
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_io_subrequest {
    pub subreq: netfs_io_subrequest,
    pub rreq: *mut netfs_io_request,
    pub req: *mut cifs_io_request,
}

//
// Take a reference on the file private data. Must be called with
// cfile->file_info_lock held.
//
extern "C" {
    pub fn cifsFileInfo_put(cifs_file: *mut cifsFileInfo);
}
pub const CIFS_CACHE_READ_FLG: c_int = 1;
pub const CIFS_CACHE_HANDLE_FLG: c_int = 2;

pub const CIFS_CACHE_WRITE_FLG: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_inode_flags {
    CIFS_INODE_PENDING_OPLOCK_BREAK,	/* oplock break in progress */
    CIFS_INODE_PENDING_WRITERS,		/* Writes in progress */
    CIFS_INODE_FLAG_UNUSED,			/* Unused flag */
    CIFS_INO_DELETE_PENDING,		/* delete pending on server */
    CIFS_INO_INVALID_MAPPING,		/* pagecache is invalid */
    CIFS_INO_LOCK,				/* lock bit for synchronization */
    CIFS_INO_TMPFILE,			/* for O_TMPFILE inodes */
    CIFS_INO_CLOSE_ON_LOCK,			/* Not to defer the close when lock is set */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifsInodeInfo {
    pub /: *mut *mut netfs_inode netfs; / Netfslib context and vfs inode,
    pub can_cache_brlcks: bool,
    pub /: *mut *mut list_head llist; / locks helb by this inode,
//
// NOTE: Some code paths call down_read(lock_sem) twice, so
// we must always use cifs_down_write() instead of down_write()
// for this semaphore to avoid deadlocks.
//
    pub /: *mut *mut rw_semaphore lock_sem; / protect the fields above,
// BB add in lists for dirty pages i.e. write caching info for oplock
    pub openFileList: list_head,
    pub /: *mut *mut spinlock_t open_file_lock; / protects openFileList,
    pub /: *mut *mut __u32 cifsAttrs; / e.g. DOS archive bit, sparse, compressed, system,
    pub /: *mut *mut unsigned int oplock; / oplock/lease level we have,
    pub /: *mut *mut __u16 epoch; / used to track lease state changes,
    pub flags: c_ulong,
    pub writers_lock: spinlock_t,
    pub /: *mut *mut unsigned int writers; / Number of writers on this inode,
    pub /: *mut *mut unsigned long time; / jiffies of last update of inode,
    pub /: *mut *mut unsigned long time_last_write; / jiffies of last writable close or truncate,
    pub /: *mut *mut u64 uniqueid; / server inode number,
    pub /: *mut *mut u64 createtime; / creation time on server,
    pub /: *mut *mut __u8 lease_key[SMB2_LEASE_KEY_SIZE]; / lease key for this inode,
    pub /: *mut *mut list_head deferred_closes; / list of deferred closes,
    pub /: *mut *mut spinlock_t deferred_lock; / protection on deferred list,
    pub /: *mut *mut bool lease_granted; / Flag to indicate whether lease or oplock is granted.,
    pub symlink_target: *mut c_char,
    pub reparse_tag: __u32,
}

extern "C" {
    pub fn container_of(_arg: inode, cifsInodeInfo: struct, _arg: netfs.inode) -> return;
}

//
// Use atomic_t for @cifs_sb->mnt_cifs_flags as it is currently accessed
// locklessly and may be changed concurrently by mount/remount and reconnect
// paths.
//
extern "C" {
    pub fn atomic_read(_arg: &cifs_sb->mnt_cifs_flags) -> return;
}
// pos = delim;

//
// This is the prototype for the mid receive function. This function is for
// receiving the rest of the SMB frame, starting with the WordCount (which is
// just after the MID in struct smb_hdr). Note:
//
// - This will be called by cifsd, with no locks held.
// - The mid will still be on the pending_mid_q.
// - mid->resp_buf will point to the current buffer.
//
// Returns zero on a successful receive, or an error. The receive state in
// the TCP_Server_Info will also be updated.
//
// This is the prototype for the mid callback function. This is called once the
// mid has been received off of the socket. When creating one, take special
// care to avoid deadlocks. Things to bear in mind:
//
// - it will be called by cifsd, with no locks held
// - the mid will be removed from any lists
//
extern "C" {
    pub fn void(srv: *mut *mut mid_callback_t)(struct TCP_Server_Info, mid: *mut mid_q_entry) -> typedef;
}
//
// This is the protopyte for mid handle function. This is called once the mid
// has been recognized after decryption of the message.
//
// one of these for every pending CIFS request to the server
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_q_entry {
    pub /: *mut *mut list_head qhead; / mids waiting on reply from this server,
    pub refcount: refcount_t,
    pub /: *mut *mut __u64 mid; / multiplex id,
    pub /: *mut *mut __u16 credits; / number of credits consumed by this mid,
    pub /: *mut *mut __u16 credits_received; / number of credits from the response,
    pub /: *mut *mut __u32 pid; / process id,
    pub /: *mut *mut __u32 sequence_number; / for CIFS signing,
    pub /: *mut *mut unsigned int sr_flags; / Flags passed to send_recv(),
    pub /: *mut *mut unsigned long when_alloc; / when mid was created,

    pub /: *mut *mut unsigned long when_sent; / time when smb send finished,
    pub /: *mut *mut unsigned long when_received; / when demux complete (taken off wire),

    pub /: *mut *mut mid_receive_t receive; / call receive callback,
    pub /: *mut *mut mid_callback_t callback; / call completion callback,
    pub /: *mut *mut mid_handle_t handle; / call handle mid callback,
    pub /: *mut *mut *mut void callback_data; / general purpose pointer for callback,
    pub creator: *mut task_struct,
    pub /: *mut *mut *mut void resp_buf; / pointer to received SMB header,
    pub resp_buf_size: c_uint,
    pub response_pdu_len: u32,
    pub /: *mut *mut int mid_state; / wish this were enum but can not pass to wait_event,
    pub /: *mut *mut int mid_rc; / rc for MID_RC,
    pub /: *mut *mut __le16 command; / smb command code,
    pub /: *mut *mut unsigned int optype; / operation type,
    pub mid_lock: spinlock_t,
    pub /: *mut *mut bool wait_cancelled:1; / Cancelled while waiting for response,
    pub /: *mut *mut bool deleted_from_q:1; / Whether Mid has been dequeued frem pending_mid_q,
    pub /: *mut *mut bool large_buf:1; / if valid response, is pointer to large buf,
    pub /: *mut *mut bool multiRsp:1; / multiple trans2 responses for one request,
    pub /: *mut *mut bool multiEnd:1; / both received,
    pub /: *mut *mut bool decrypted:1; / decrypted entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct close_cancelled_open {
    pub fid: cifs_fid,
    pub tcon: *mut cifs_tcon,
    pub work: work_struct,
    pub mid: __u64,
    pub cmd: __u16,
}

// Make code in transport.c a little cleaner by moving

// for pending dnotify requests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dir_notify_req {
    pub lhead: list_head,
    pub Pid: __le16,
    pub PidHigh: __le16,
    pub Mid: __u16,
    pub Tid: __u16,
    pub Uid: __u16,
    pub netfid: __u16,
    pub /: *mut *mut __u32 filter; / CompletionFilter (for multishot),
    pub multishot: c_int,
    pub pfile: *mut file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfs_info3_param {
    pub DFSREF_STORAGE_SERVER*/: *mut *mut int flags; / DFSREF_REFERRAL_SERVER,,
    pub path_consumed: c_int,
    pub server_type: c_int,
    pub ref_flag: c_int,
    pub path_name: *mut c_char,
    pub node_name: *mut c_char,
    pub ttl: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_list {
    pub list: list_head,
    pub cfile: *mut cifsFileInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_mount_ctx {
    pub cifs_sb: *mut cifs_sb_info,
    pub fs_ctx: *mut smb3_fs_context,
    pub xid: c_uint,
    pub server: *mut TCP_Server_Info,
    pub ses: *mut cifs_ses,
    pub tcon: *mut cifs_tcon,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchan_mount {
    pub work: work_struct,
    pub ses: *mut cifs_ses,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_find_flags {
    FIND_ANY		= 0U,
    FIND_FSUID_ONLY		= (1U << 0),
    FIND_WITH_DELETE	= (1U << 1),
    FIND_NO_PENDING_DELETE	= (1U << 2),
    FIND_OPEN_FLAGS		= (1U << 3),
}

pub const MID_FREE: c_int = 0;
pub const MID_REQUEST_ALLOCATED: c_int = 1;
pub const MID_REQUEST_SUBMITTED: c_int = 2;
pub const MID_RESPONSE_RECEIVED: c_int = 4;

pub const MID_RESPONSE_MALFORMED: c_uint = 0x10;
pub const MID_SHUTDOWN: c_uint = 0x20;
pub const MID_RESPONSE_READY: c_uint = 0x40 /* ready for other process handle the rsp */;
pub const MID_RC: c_uint = 0x80 /* mid_rc contains custom rc */;
// Types of response buffer returned from SendReceive2

pub const CIFS_SMALL_BUFFER: c_int = 1;
pub const CIFS_LARGE_BUFFER: c_int = 2;

// Type of Request to SendReceive2

pub const CIFS_TIMEOUT_MASK: c_uint = 0x003    /* only one of above set in req */;
pub const CIFS_LOG_ERROR: c_uint = 0x010    /* log NT STATUS if non-zero */;
pub const CIFS_LARGE_BUF_OP: c_uint = 0x020    /* large request buffer */;
pub const CIFS_NO_RSP_BUF: c_uint = 0x040    /* no response buffer required */;
// Type of request operation
pub const CIFS_ECHO_OP: c_uint = 0x080  /* echo request */;
pub const CIFS_OBREAK_OP: c_uint = 0x0100 /* oplock break request */;
pub const CIFS_NEG_OP: c_uint = 0x0200 /* negotiate request */;
pub const CIFS_CP_CREATE_CLOSE_OP: c_uint = 0x0400 /* compound create+close request */;
// Lower bitmask values are reserved by others below.
pub const CIFS_SESS_OP: c_uint = 0x2000 /* session setup request */;
pub const CIFS_OP_MASK: c_uint = 0x2780 /* mask request type */;
pub const CIFS_HAS_CREDITS: c_uint = 0x0400 /* already has credits */;
pub const CIFS_TRANSFORM_REQ: c_uint = 0x0800 /* transform request before sending */;
pub const CIFS_NO_SRV_RSP: c_uint = 0x1000 /* there is no server response */;
pub const CIFS_COMPRESS_REQ: c_uint = 0x4000 /* compress request before sending */;
pub const CIFS_INTERRUPTIBLE_WAIT: c_uint = 0x8000 /* Interruptible wait (e.g. lock request) */;
pub const CIFS_WINDOWS_LOCK: c_uint = 0x10000 /* We're trying to get a Windows lock */;
// Security Flags: indicate type of session setup needed
pub const CIFSSEC_MAY_SIGN: c_uint = 0x00001;
pub const CIFSSEC_MAY_NTLMV2: c_uint = 0x00004;
pub const CIFSSEC_MAY_KRB5: c_uint = 0x00008;
pub const CIFSSEC_MAY_SEAL: c_uint = 0x00040;
pub const CIFSSEC_MAY_NTLMSSP: c_uint = 0x00080 /* raw ntlmssp with ntlmv2 */;
pub const CIFSSEC_MUST_SIGN: c_uint = 0x01001;
// note that only one of the following can be set so the
pub const CIFSSEC_MUST_NTLMV2: c_uint = 0x04004;
pub const CIFSSEC_MUST_KRB5: c_uint = 0x08008;

pub const CIFSSEC_MASK: c_uint = 0xCF0CF /* flags supported if no weak allowed */;

pub const CIFSSEC_MASK: c_uint = 0xC70C7 /* flags supported if no weak allowed */;

pub const CIFSSEC_MUST_SEAL: c_uint = 0x40040;
pub const CIFSSEC_MUST_NTLMSSP: c_uint = 0x80080 /* raw ntlmssp with ntlmv2 */;

//
// All constants go here
//

//
// Note that ONE module should define _DECLARE_GLOBALS_HERE to cause the
// following to be declared.
//
// LOCK ORDERING NOTES:
//
// Here are all the locks (spinlock, mutex, semaphore) in cifs.ko, arranged according
// to the locking order. i.e. if two locks are to be held together, the lock that
// appears higher in this list needs to be taken before the other.
//
// If you hold a lock that is lower in this list, and you need to take a higher lock
// (or if you think that one of the functions that you're calling may need to), first
// drop the lock you hold, pick up the higher lock, then the lower one. This will
// ensure that locks are picked up only in one direction in the below table
// (top to bottom).
//
// Also, if you expect a function to be called with a lock held, explicitly document
// this in the comments on top of your function definition.
//
// And also, try to keep the critical sections (lock hold time) to be as minimal as
// possible. Blocking / calling other functions with a lock held always increase
// the risk of a possible deadlock.
//
// Following this rule will avoid unnecessary deadlocks, which can get really hard to
// debug. Also, any new lock that you introduce, please add to this list in the correct
// order.
//
// Please populate this list whenever you introduce new locks in your changes. Or in
// case I've missed some existing locks. Please ensure that it's added in the list
// based on the locking order expected.
//
// =====================================================================================
// Lock				Protects			Initialization fn
// =====================================================================================
// cifs_mount_mutex		mount/unmount operations
// vol_list_lock
// vol_info->ctx_lock		vol_info->ctx
// cifs_sb_info->tlink_tree_lock	cifs_sb_info->tlink_tree	cifs_setup_cifs_sb
// TCP_Server_Info->		TCP_Server_Info			cifs_get_tcp_session
// reconnect_mutex
// cifs_ses->session_mutex	cifs_ses			sesInfoAlloc
// TCP_Server_Info->srv_mutex	TCP_Server_Info			cifs_get_tcp_session
// cifs_tcp_ses_lock		cifs_tcp_ses_list		sesInfoAlloc
// cifs_tcon->open_file_lock	cifs_tcon->openFileList		tconInfoAlloc
// cifs_tcon->pending_opens
// cifs_tcon->stat_lock		cifs_tcon->bytes_read		tconInfoAlloc
// cifs_tcon->bytes_written
// cifs_tcon->fscache_lock	cifs_tcon->fscache		tconInfoAlloc
// cifs_tcon->sb_list_lock	cifs_tcon->cifs_sb_list		tconInfoAlloc
// GlobalMid_Lock		GlobalMaxActiveXid		init_cifs
// GlobalCurrentXid
// GlobalTotalActiveXid
// TCP_Server_Info->srv_lock	(anything in struct not protected by another lock and can change)
// TCP_Server_Info->mid_queue_lock	TCP_Server_Info->pending_mid_q	cifs_get_tcp_session
// mid_q_entry->deleted_from_q
// TCP_Server_Info->mid_counter_lock    TCP_Server_Info->current_mid    cifs_get_tcp_session
// TCP_Server_Info->req_lock	TCP_Server_Info->in_flight	cifs_get_tcp_session
// ->credits
// ->echo_credits
// ->oplock_credits
// ->reconnect_instance
// cifs_ses->ses_lock		(anything that is not protected by another lock and can change)
// sesInfoAlloc
// cifs_ses->iface_lock		cifs_ses->iface_list		sesInfoAlloc
// ->iface_count
// ->iface_last_update
// cifs_ses->chan_lock		cifs_ses->chans			sesInfoAlloc
// ->chans_need_reconnect
// ->chans_in_reconnect
// cifs_tcon->tc_lock		(anything that is not protected by another lock and can change)
// tcon_info_alloc
// cifs_swnreg_idr_mutex	cifs_swnreg_idr			cifs_swn.c
// (witness service registration, accesses tcon fields under tc_lock)
// inode->i_rwsem, taken by fs/netfs/locking.c e.g. should be taken before cifsInodeInfo locks
// cifsInodeInfo->open_file_lock	cifsInodeInfo->openFileList	cifs_alloc_inode
// cifsInodeInfo->writers_lock	cifsInodeInfo->writers		cifsInodeInfo_alloc
// cifsInodeInfo->lock_sem	cifsInodeInfo->llist		cifs_init_once
// ->can_cache_brlcks
// cifsInodeInfo->deferred_lock	cifsInodeInfo->deferred_closes	cifsInodeInfo_alloc
// cached_fids->cfid_list_lock	cifs_tcon->cfids->entries	init_cached_dirs
// cached_fid->dirents.de_mutex	cached_fid->dirents		alloc_cached_dir
// cifsFileInfo->fh_mutex	cifsFileInfo			cifs_new_fileinfo
// cifsFileInfo->file_info_lock	cifsFileInfo->count		cifs_new_fileinfo
// ->invalidHandle			initiate_cifs_search
// ->oplock_break_cancelled
// smbdirect_mr->mutex		RDMA memory region management	(SMBDirect only)
// mid_q_entry->mid_lock	mid_q_entry->callback           alloc_mid
// smb2_mid_entry_alloc
// (Any fields of mid_q_entry that will need protection)
//

// Macro flag: #define GLOBAL_EXTERN

//
// the list of TCP_Server_Info structures, ie each of the sockets
// connecting our client to a distinct server (ip address), is
// chained together by cifs_tcp_ses_list. The list of all our SMB
// sessions (and from that the tree connections) can be found
// by iterating over cifs_tcp_ses_list
//
// This lock protects the cifs_tcp_ses_list, the list of smb sessions per
// tcp session, and the list of tcon's per smb session. It also protects
// the reference counters for the server, smb session, and tcon.
// generally the locks should be taken in order tcp_ses_lock before
// tcon->open_file_lock and that before file->file_info_lock since the
// structure order is cifs_socket-->cifs_ses-->cifs_tcon-->cifs_file
//
// Global transaction id (XID) information
//
// Global counters, updated atomically
//
// Various Debug counters

// Misc globals
extern "C" {
    pub fn cifs_oplock_break(work: *mut work_struct);
}
extern "C" {
    pub fn cifs_queue_oplock_break(cfile: *mut cifsFileInfo);
}
extern "C" {
    pub fn smb2_deferred_work_close(work: *mut work_struct);
}
// Operations for different SMB versions

// extern struct smb_version_operations smb302_operations;*/ /* not needed yet
//
// For SMB1, see MS-CIFS 2.4.55 SMB_COM_TREE_CONNECT_ANDX (0x75) and MS-CIFS 3.3.4.4 DFS
// Subsystem Notifies That a Share Is a DFS Share.
//
// For SMB2+, see MS-SMB2 2.2.10 SMB2 TREE_CONNECT Response and MS-SMB2 3.3.4.14 Server
// Application Updates a Share.
//
// Check if all targets are capable of handling DFS referrals as per
// MS-DFSC 2.2.4 RESP_GET_DFS_REFERRAL.
//
extern "C" {
    pub fn is_tcon_dfs(DFSREF_REFERRAL_SERVER): tcon) || (ref && (ref->flags &) -> return;
}
//
// Make workstation name no more than 15 chars when using insecure dialects as some legacy
// servers do require it during NTLMSSP.
//
extern "C" {
    pub fn min_t(_arg: usize, _arg: sizeof(ses->workstation_name), _arg: RFC1001_NAME_LEN_WITH_NULL) -> return;
}
extern "C" {
    pub fn sizeof(_arg: ses->workstation_name) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_compound_vars {
    pub oparms: cifs_open_parms,
    pub rsp_iov: [kvec; MAX_COMPOUND],
    pub rqst: [smb_rqst; MAX_COMPOUND],
    pub open_iov: [kvec; SMB2_CREATE_IOV_SIZE],
    pub qi_iov: kvec,
    pub io_iov: [kvec; SMB2_IOCTL_IOV_SIZE],
    pub si_iov: [kvec; SMB2_SET_INFO_IOV_SIZE],
    pub hl_iov: [kvec; SMB2_SET_INFO_IOV_SIZE],
    pub unlink_iov: [kvec; SMB2_SET_INFO_IOV_SIZE],
    pub rename_iov: [kvec; SMB2_SET_INFO_IOV_SIZE],
    pub close_iov: kvec,
    pub rename_info: smb2_file_rename_info_hdr,
    pub link_info: smb2_file_link_info_hdr,
    pub ea_iov: kvec,
}

//
// Execute mid callback atomically - ensures callback runs exactly once
// and prevents sleeping in atomic context.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_calc_sig_ctx {
    pub md5: *mut md5_ctx,
    pub hmac: *mut hmac_sha256_ctx,
    pub cmac: *mut aes_cmac_ctx,
}

pub const CIFS_RECONN_DELAY_SECS: c_int = 30;

// O_SYNC also has bit for O_DSYNC so following check picks up either
//
// inode->i_blocks is counted in 512-byte units, independent of
// inode->i_blksize.
//

