//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/smb2pdu.h
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
// Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//

// Create Action Flags
pub const FILE_SUPERSEDED: c_uint = 0x00000000;
pub const FILE_OPENED: c_uint = 0x00000001;
pub const FILE_CREATED: c_uint = 0x00000002;
pub const FILE_OVERWRITTEN: c_uint = 0x00000003;
// SMB2 Max Credits
pub const SMB2_MAX_CREDITS: c_int = 8192;
// BB FIXME - analyze following length BB
pub const MAX_SMB2_HDR_SIZE: c_uint = 0x78 /* 4 len + 64 hdr + (2*24 wct) + 2 bct + 2 pad */;

//
// Definitions for SMB2 Protocol Data Units (network frames)
//
// See MS-SMB2.PDF specification for protocol details.
// The Naming convention is the lower case version of the SMB2
// command code name for the struct. Note that structures must be packed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct preauth_integrity_info {
// PreAuth integrity Hash ID
    pub Preauth_HashId: __le16,
// PreAuth integrity Hash Value
    pub Preauth_HashValue: [__u8; SMB2_PREAUTH_HASH_SIZE],
}

// offset is sizeof smb2_negotiate_rsp but rounded up to 8 bytes.

// sizeof(struct smb2_negotiate_rsp) =
// header(64) + response(64) + GSS_LENGTH(96) + GSS_PADDING(0)
//
pub const OFFSET_OF_NEG_CONTEXT: c_uint = 0xe0;

// sizeof(struct smb2_negotiate_rsp) =
// header(64) + response(64) + GSS_LENGTH(74) + GSS_PADDING(6)
//
pub const OFFSET_OF_NEG_CONTEXT: c_uint = 0xd0;

// Apple Defined Contexts

//
// AAPL SMB2 extension -- kAAPL_SERVER_QUERY create context.
//
// Command code and bitmap values are the existing
// SMB2_CRTCTX_AAPL_* constants in fs/smb/common/smb2pdu.h.
//
// Omitting the model string when reply_bitmap includes
// SMB2_CRTCTX_AAPL_MODEL_INFO causes smbfs.kext to enter a broken
// disconnect path requiring a reboot.
//
// Layout: ccontext(16) + Name[4] + Pad[4] + cmd(4) + reserved(4) +
// reply_bitmap(8) + server_caps(8) + vol_caps(8)
// When MODEL_INFO requested, appended: pad2(4) + model_bytes(4) + UTF-16LE
//
pub const SMB2_CREATE_AAPL_LEN: c_int = 4;
//
// Server capability flags (server_caps field) -- SMB2_CRTCTX_AAPL_UNIX_BASED:
// prevents macOS Windows-compat mode (question-mark icons).
// SMB2_CRTCTX_AAPL_SUPPORTS_OSX_COPYFILE: enables server-side file copy via
// FSCTL_SRV_COPYCHUNK. SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR: inline
// FinderInfo per FIND entry, set when client also advertises the bit;
// format: EaSize=max_access, ShortName[0..7]=rfork_size,
// ShortName[8..23]=FinderInfo(16B), Reserved2=unix_mode.
//

//
// READDIR_ATTR_V2 (SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR_V2, see
// fs/smb/common/smb2pdu.h) extends the same inline-FinderInfo mechanism
// above with a flags field, confirmed byte-identical to V1 otherwise
// against AAPL's actual public client behavior. When a client's own
// client_caps requests V2, the server advertises V2 instead of V1 in
// its own server_caps reply; V1 and V2 are mutually exclusive on the
// wire, not both set together. The wire format's ShortNameLength+Reserved
// (ignored in V1) become a single flags field in V2 --
// AAPL_READDIR_ATTR_V2_NO_XATTR is the only flag bit currently defined,
// signaling the item has no xattrs/streams so the client can skip a
// separate query.
//
pub const AAPL_READDIR_ATTR_V2_NO_XATTR: c_uint = 0x01;
// Model string: up to 31 ASCII chars
pub const AAPL_MODEL_MAX_CHARS: c_int = 31;

//
// Max AAPL response: header(24) + base data(32) + pad2(4) + model_bytes(4)
// + model(62), 8-byte aligned: ALIGN(126, 8) = 128 bytes.
//
pub const AAPL_RSP_MAX_SIZE: c_int = 128;
// AAPL server query request (client->server)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aapl_server_query_req {
    pub cmd: __le32,
    pub reserved: __le32,
    pub req_bitmap: __le64,
    pub client_caps: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_aapl_rsp {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 4],
    pub Pad: [__u8; 4],
    pub cmd: __le32,
    pub reserved: __le32,
    pub reply_bitmap: __le64,
    pub server_caps: __le64,
    pub vol_caps: __le64,
// when MODEL_INFO requested: __le32 pad2; __le32 model_bytes; __le16 model[]
    pub __packed: },
pub const DURABLE_HANDLE_MAX_TIMEOUT: c_int = 300000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_alloc_size_req {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub AllocationSize: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_durable_rsp {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub Reserved: [__u8; 8],
    pub data: __u64,
    pub Data: },
    pub __packed: },
//
// See POSIX-SMB2 2.2.14.2.16
// Link: https://gitlab.com/samba-team/smb3-posix-spec/-/blob/master/smb3_posix_extensions.md
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_posix_rsp {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 16],
    pub nlink: __le32,
    pub reparse_tag: __le32,
    pub mode: __le32,
// SidBuffer contain two sids(Domain sid(28), UNIX group sid(16))
    pub SidBuffer: [u8; 44],
    pub __packed: },
pub const SMB2_0_IOCTL_IS_FSCTL: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage_rsp {
    pub Family: __le16,
    pub addr4: smb_sockaddr_in,
    pub addr6: smb_sockaddr_in6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_object_buf_type1_ioctl_rsp {
    pub ObjectId: [__u8; 16],
    pub BirthVolumeId: [__u8; 16],
    pub BirthObjectId: [__u8; 16],
    pub DomainId: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_sparse {
    pub SetSparse: __u8,
    pub __packed: },
// FILE Info response size
pub const FILE_DIRECTORY_INFORMATION_SIZE: c_int = 1;
pub const FILE_FULL_DIRECTORY_INFORMATION_SIZE: c_int = 2;
pub const FILE_BOTH_DIRECTORY_INFORMATION_SIZE: c_int = 3;
pub const FILE_BASIC_INFORMATION_SIZE: c_int = 40;
pub const FILE_STANDARD_INFORMATION_SIZE: c_int = 24;
pub const FILE_INTERNAL_INFORMATION_SIZE: c_int = 8;
pub const FILE_EA_INFORMATION_SIZE: c_int = 4;
pub const FILE_ACCESS_INFORMATION_SIZE: c_int = 4;
pub const FILE_NAME_INFORMATION_SIZE: c_int = 9;
pub const FILE_RENAME_INFORMATION_SIZE: c_int = 10;
pub const FILE_LINK_INFORMATION_SIZE: c_int = 11;
pub const FILE_NAMES_INFORMATION_SIZE: c_int = 12;
pub const FILE_DISPOSITION_INFORMATION_SIZE: c_int = 13;
pub const FILE_POSITION_INFORMATION_SIZE: c_int = 14;
pub const FILE_FULL_EA_INFORMATION_SIZE: c_int = 15;
pub const FILE_MODE_INFORMATION_SIZE: c_int = 4;
pub const FILE_ALIGNMENT_INFORMATION_SIZE: c_int = 4;
pub const FILE_ALL_INFORMATION_SIZE: c_int = 104;
pub const FILE_ALLOCATION_INFORMATION_SIZE: c_int = 19;
pub const FILE_END_OF_FILE_INFORMATION_SIZE: c_int = 20;
pub const FILE_ALTERNATE_NAME_INFORMATION_SIZE: c_int = 8;
pub const FILE_STREAM_INFORMATION_SIZE: c_int = 32;
pub const FILE_PIPE_INFORMATION_SIZE: c_int = 23;
pub const FILE_PIPE_LOCAL_INFORMATION_SIZE: c_int = 24;
pub const FILE_PIPE_REMOTE_INFORMATION_SIZE: c_int = 25;
pub const FILE_MAILSLOT_QUERY_INFORMATION_SIZE: c_int = 26;
pub const FILE_MAILSLOT_SET_INFORMATION_SIZE: c_int = 27;
pub const FILE_COMPRESSION_INFORMATION_SIZE: c_int = 16;
pub const FILE_OBJECT_ID_INFORMATION_SIZE: c_int = 29;
// Number 30 not defined in documents
pub const FILE_MOVE_CLUSTER_INFORMATION_SIZE: c_int = 31;
pub const FILE_QUOTA_INFORMATION_SIZE: c_int = 32;
pub const FILE_REPARSE_POINT_INFORMATION_SIZE: c_int = 33;
pub const FILE_NETWORK_OPEN_INFORMATION_SIZE: c_int = 56;
pub const FILE_ATTRIBUTE_TAG_INFORMATION_SIZE: c_int = 8;
// FS Info response  size
pub const FS_DEVICE_INFORMATION_SIZE: c_int = 8;
pub const FS_ATTRIBUTE_INFORMATION_SIZE: c_int = 16;
pub const FS_VOLUME_INFORMATION_SIZE: c_int = 24;
pub const FS_SIZE_INFORMATION_SIZE: c_int = 24;
pub const FS_FULL_SIZE_INFORMATION_SIZE: c_int = 32;
pub const FS_SECTOR_SIZE_INFORMATION_SIZE: c_int = 28;
pub const FS_OBJECT_ID_INFORMATION_SIZE: c_int = 64;
pub const FS_CONTROL_INFORMATION_SIZE: c_int = 48;
pub const FS_POSIX_INFORMATION_SIZE: c_int = 56;
// FS_ATTRIBUTE_File_System_Name
pub const FS_TYPE_SUPPORT_SIZE: c_int = 44;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_type_info {
    pub fs_name: *mut c_char,
    pub magic_number: c_long,
    pub __packed: },
//
// PDU query infolevel structure definitions
// BB consider moving to a different header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_access_info {
    pub AccessFlags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_alignment_info {
    pub AlignmentRequirement: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_alt_name_info {
    pub FileNameLength: __le32,
    pub FileName: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_stream_info {
    pub NextEntryOffset: __le32,
    pub StreamNameLength: __le32,
    pub StreamSize: __le64,
    pub StreamAllocationSize: __le64,
    pub StreamName: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srv_snapshot_array {
    pub NumberOfSnapShots: __le32,
    pub NumberOfSnapShotsReturned: __le32,
    pub SnapShotArraySize: __le32,
    pub Reserved: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_standard_info {
    pub AllocationSize: __le64,
    pub EndOfFile: __le64,
    pub /: *mut *mut __le32 NumberOfLinks; / hard links,
    pub DeletePending: __u8,
    pub Directory: __u8,
    pub Reserved: __le16,
    pub /: *mut *mut } __packed; / level 18 Query,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_ea_info {
    pub EASize: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_disposition_info {
    pub DeletePending: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_pos_info {
    pub CurrentByteOffset: __le64,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_mode_info {
    pub Mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_comp_info {
    pub CompressedFileSize: __le64,
    pub CompressionFormat: __le16,
    pub CompressionUnitShift: __u8,
    pub ChunkShift: __u8,
    pub ClusterShift: __u8,
    pub Reserved: [__u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_attr_tag_info {
    pub FileAttributes: __le32,
    pub ReparseTag: __le32,
    pub __packed: },
pub const SL_RESTART_SCAN: c_uint = 0x00000001;
pub const SL_RETURN_SINGLE_ENTRY: c_uint = 0x00000002;
pub const SL_INDEX_SPECIFIED: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_ea_info_req {
    pub NextEntryOffset: __le32,
    pub EaNameLength: __u8,
    pub name: [c_char; ],
    pub /: *mut *mut } __packed; / level 15 Query,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_ea_info {
    pub NextEntryOffset: __le32,
    pub Flags: __u8,
    pub EaNameLength: __u8,
    pub EaValueLength: __le16,
    pub name: [c_char; ],
// optionally followed by value
    pub /: *mut *mut } __packed; / level 15 Query,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_ea_buf_req {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub ea: smb2_ea_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_sd_buf_req {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub ntsd: smb_ntsd,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_posix_info {
    pub NextEntryOffset: __le32,
    pub Ignored: __u32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub EndOfFile: __le64,
    pub AllocationSize: __le64,
    pub DosAttributes: __le32,
    pub Inode: __le64,
    pub DeviceId: __le32,
    pub Zero: __le32,
// beginning of POSIX Create Context Response
    pub HardLinks: __le32,
    pub ReparseTag: __le32,
    pub Mode: __le32,
// SidBuffer contain two sids (UNIX user sid(16), UNIX group sid(16))
    pub SidBuffer: [u8; 32],
    pub name_len: __le32,
    pub name: [u8; ],
//
// var sized owner SID
// var sized group SID
// le32 filenamelength
// u8  filename[]
//
    pub __packed: },
// functions
    pub conn): *mut void init_smb2_1_server(struct ksmbd_conn,
    pub conn): *mut void init_smb3_0_server(struct ksmbd_conn,
    pub conn): *mut void init_smb3_02_server(struct ksmbd_conn,
    pub conn): *mut int init_smb3_11_server(struct ksmbd_conn,
    pub sz): void init_smb2_max_read_size(unsigned int,
    pub sz): void init_smb2_max_write_size(unsigned int,
    pub sz): void init_smb2_max_trans_size(unsigned int,
    pub sz): void init_smb2_max_credits(unsigned int,
    pub work): *mut bool is_smb2_neg_cmd(struct ksmbd_work,
    pub work): *mut bool is_smb2_rsp(struct ksmbd_work,
    pub work): *mut u16 get_smb2_cmd_val(struct ksmbd_work,
    pub err): *mut *mut void set_smb2_rsp_status(struct ksmbd_work work, __le32,
    pub work): *mut int init_smb2_rsp_hdr(struct ksmbd_work,
    pub work): *mut int smb2_allocate_rsp_buf(struct ksmbd_work,
    pub work): *mut bool is_chained_smb2_message(struct ksmbd_work,
    pub work): *mut int init_smb2_neg_rsp(struct ksmbd_work,
    pub work): *mut void smb2_set_err_rsp(struct ksmbd_work,
    pub work): *mut int smb2_check_user_session(struct ksmbd_work,
    pub work): *mut int smb2_get_ksmbd_tcon(struct ksmbd_work,
    pub command): *mut *mut bool smb2_is_sign_req(struct ksmbd_work work, unsigned int,
    pub work): *mut int smb2_check_sign_req(struct ksmbd_work,
    pub work): *mut void smb2_set_sign_rsp(struct ksmbd_work,
    pub work): *mut int smb3_check_sign_req(struct ksmbd_work,
    pub work): *mut void smb3_set_sign_rsp(struct ksmbd_work,
    pub dialects_count): __le16,
    pub f): *mut *mut file_lock smb_flock_init(file,
    pub arg): *mut c_void,
    pub work): *mut void release_async_work(struct ksmbd_work,
    pub status): *mut *mut void smb2_send_interim_resp(struct ksmbd_work work, __le32,
    pub conn): *mut ksmbd_conn,
    pub work): *mut void smb3_preauth_hash_rsp(struct ksmbd_work,
    pub buf): *mut bool smb3_is_transform_hdr(void,
    pub work): *mut int smb3_decrypt_req(struct ksmbd_work,
    pub work): *mut int smb3_encrypt_resp(struct ksmbd_work,
    pub work): *mut bool smb3_11_final_sess_setup_resp(struct ksmbd_work,
    pub work): *mut int smb2_set_rsp_credits(struct ksmbd_work,
    pub conn): *mut bool smb3_encryption_negotiated(struct ksmbd_conn,
// smb2 misc functions
    pub work): *mut int ksmbd_smb2_check_message(struct ksmbd_work,
    pub work): *mut void smb2_complete_request_open(struct ksmbd_work,
// smb2 command handlers
    pub work): *mut int smb2_handle_negotiate(struct ksmbd_work,
    pub work): *mut int smb2_negotiate_request(struct ksmbd_work,
    pub work): *mut int smb2_sess_setup(struct ksmbd_work,
    pub work): *mut int smb2_tree_connect(struct ksmbd_work,
    pub work): *mut int smb2_tree_disconnect(struct ksmbd_work,
    pub work): *mut int smb2_session_logoff(struct ksmbd_work,
    pub work): *mut int smb2_open(struct ksmbd_work,
    pub work): *mut int smb2_query_info(struct ksmbd_work,
    pub work): *mut int smb2_query_dir(struct ksmbd_work,
    pub work): *mut int smb2_close(struct ksmbd_work,
    pub work): *mut int smb2_echo(struct ksmbd_work,
    pub work): *mut int smb2_set_info(struct ksmbd_work,
    pub work): *mut int smb2_read(struct ksmbd_work,
    pub work): *mut int smb2_write(struct ksmbd_work,
    pub work): *mut int smb2_flush(struct ksmbd_work,
    pub work): *mut int smb2_cancel(struct ksmbd_work,
    pub work): *mut int smb2_lock(struct ksmbd_work,
    pub work): *mut int smb2_ioctl(struct ksmbd_work,
    pub work): *mut int smb2_oplock_break(struct ksmbd_work,
    pub ksmbd_work): *mut int smb2_notify(struct ksmbd_work,
pub const POSIX_TYPE_FILE: c_int = 0;
pub const POSIX_TYPE_DIR: c_int = 1;
pub const POSIX_TYPE_SYMLINK: c_int = 2;
pub const POSIX_TYPE_CHARDEV: c_int = 3;
pub const POSIX_TYPE_BLKDEV: c_int = 4;
pub const POSIX_TYPE_FIFO: c_int = 5;
pub const POSIX_TYPE_SOCKET: c_int = 6;
pub const POSIX_FILETYPE_SHIFT: c_int = 12;
