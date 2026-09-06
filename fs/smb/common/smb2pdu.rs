//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/common/smb2pdu.h
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
// Note that, due to trying to use names similar to the protocol specifications,
// there are many mixed case field names in the structures below.  Although
// this does not match typical Linux kernel style, it is necessary to be
// able to match against the protocol specification.
//
// SMB2 commands
// Some commands have minimal (wct=0,bcc=0), or uninteresting, responses
// (ie no useful data other than the SMB error code itself) and are marked such.
// Knowing this helps avoid response buffer allocations and copy in some cases.
//
// List of commands in host endian
pub const SMB2_NEGOTIATE_HE: c_uint = 0x0000;
pub const SMB2_SESSION_SETUP_HE: c_uint = 0x0001;
pub const SMB2_LOGOFF_HE: c_uint = 0x0002 /* trivial request/resp */;
pub const SMB2_TREE_CONNECT_HE: c_uint = 0x0003;
pub const SMB2_TREE_DISCONNECT_HE: c_uint = 0x0004 /* trivial req/resp */;
pub const SMB2_CREATE_HE: c_uint = 0x0005;
pub const SMB2_CLOSE_HE: c_uint = 0x0006;
pub const SMB2_FLUSH_HE: c_uint = 0x0007 /* trivial resp */;
pub const SMB2_READ_HE: c_uint = 0x0008;
pub const SMB2_WRITE_HE: c_uint = 0x0009;
pub const SMB2_LOCK_HE: c_uint = 0x000A;
pub const SMB2_IOCTL_HE: c_uint = 0x000B;
pub const SMB2_CANCEL_HE: c_uint = 0x000C;
pub const SMB2_ECHO_HE: c_uint = 0x000D;
pub const SMB2_QUERY_DIRECTORY_HE: c_uint = 0x000E;
pub const SMB2_CHANGE_NOTIFY_HE: c_uint = 0x000F;
pub const SMB2_QUERY_INFO_HE: c_uint = 0x0010;
pub const SMB2_SET_INFO_HE: c_uint = 0x0011;
pub const SMB2_OPLOCK_BREAK_HE: c_uint = 0x0012;
pub const SMB2_SERVER_TO_CLIENT_NOTIFICATION: c_uint = 0x0013;
// The same list in little endian

pub const NUMBER_OF_SMB2_COMMANDS: c_uint = 0x0013;
//
// Size of the session key (crypto key encrypted with the password
//
pub const SMB2_NTLMV2_SESSKEY_SIZE: c_int = 16;
pub const SMB2_SIGNATURE_SIZE: c_int = 16;
pub const SMB2_HMACSHA256_SIZE: c_int = 32;
pub const SMB2_CMACAES_SIZE: c_int = 16;
pub const SMB3_GCM128_CRYPTKEY_SIZE: c_int = 16;
pub const SMB3_GCM256_CRYPTKEY_SIZE: c_int = 32;
//
// Size of the smb3 encryption/decryption keys
// This size is big enough to store any cipher key types.
//
pub const SMB3_ENC_DEC_KEY_SIZE: c_int = 32;
//
// Size of the smb3 signing key
//
pub const SMB3_SIGN_KEY_SIZE: c_int = 16;
pub const CIFS_CLIENT_CHALLENGE_SIZE: c_int = 8;
// Maximum buffer size value we can send with 1 credit
pub const SMB2_MAX_BUFFER_SIZE: c_int = 65536;
//
// The default wsize is 1M for SMB2 (and for some CIFS cases).
// find_get_pages seems to return a maximum of 256
// pages in a single call. With PAGE_SIZE == 4k, this means we can
// fill a single wsize request with a single call.
//

// According to MS-SMB2 specification The minimum recommended value is 65536.

//
// SMB2 Header Definition
//
// "MBZ" :  Must be Zero
// "BB"  :  BugBug, Something to check/review/analyze later
// "PDU" :  "Protocol Data Unit" (ie a network "frame")
//
pub const __SMB2_HEADER_STRUCTURE_SIZE: c_int = 64;

//
// SMB2 flag definitions
//

//
// Definitions for SMB2 Protocol Data Units (network frames)
//
// See MS-SMB2.PDF specification for protocol details.
// The Naming convention is the lower case version of the SMB2
// command code name for the struct. Note that structures must be packed.
//
// See MS-SMB2 section 2.2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_hdr {
    pub /: *mut *mut __le32 ProtocolId; / 0xFE 'S' 'M' 'B',
    pub /: *mut *mut __le16 StructureSize; / 64,
    pub /: *mut *mut __le16 CreditCharge; / MBZ,
    pub /: *mut *mut __le32 Status; / Error from server,
    pub Command: __le16,
    pub /: *mut *mut __le16 CreditRequest; / CreditResponse,
    pub Flags: __le32,
    pub NextCommand: __le32,
    pub MessageId: __le64,
    pub ProcessId: __le32,
    pub TreeId: __le32,
    pub SyncId: } __packed,
    pub AsyncId: __le64,
    pub Id: } __packed,
    pub SessionId: __le64,
    pub Signature: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_hdr_req {
    pub /: *mut *mut __le32 ProtocolId; / 0xFE 'S' 'M' 'B',
    pub /: *mut *mut __le16 StructureSize; / 64,
    pub /: *mut *mut __le16 CreditCharge; / MBZ,
    pub /: *mut *mut __le16 ChannelSequence; / See MS-SMB2 3.2.4.1 and 3.2.7.1,
    pub Reserved: __le16,
    pub Command: __le16,
    pub /: *mut *mut __le16 CreditRequest; / CreditResponse,
    pub Flags: __le32,
    pub NextCommand: __le32,
    pub MessageId: __le64,
    pub ProcessId: __le32,
    pub TreeId: __le32,
    pub SyncId: } __packed,
    pub AsyncId: __le64,
    pub Id: } __packed,
    pub SessionId: __le64,
    pub Signature: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_pdu {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize2; / size of wct area (varies, request specific),
    pub __packed: },
pub const SMB2_ERROR_STRUCTURE_SIZE2: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_err_rsp {
    pub hdr: smb2_hdr,
    pub StructureSize: __le16,
    pub ErrorContextCount: __u8,
    pub Reserved: __u8,
    pub /: *mut *mut __le32 ByteCount; / even if zero, at least one byte follows,
    pub /: *mut *mut __u8 ErrorData[]; / variable length,
    pub __packed: },
pub const SMB3_AES_CCM_NONCE: c_int = 11;
pub const SMB3_AES_GCM_NONCE: c_int = 12;
// Transform flags (for 3.0 dialect this flag indicates CCM
pub const TRANSFORM_FLAG_ENCRYPTED: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_transform_hdr {
    pub /: *mut *mut __le32 ProtocolId; / 0xFD 'S' 'M' 'B',
    pub Signature: [__u8; 16],
    pub Nonce: [__u8; 16],
    pub OriginalMessageSize: __le32,
    pub Reserved1: __u16,
    pub /: *mut *mut __le16 Flags; / EncryptionAlgorithm for 3.0, enc enabled for 3.1.1,
    pub SessionId: __le64,
    pub __packed: },
//
// These are simplified versions from the spec, as we don't need a fully fledged
// form of both unchained and chained structs.
//
// For chained payloads, only the first 8 bytes belong to the transform header.
// CompressionAlgorithm, Flags and Offset below overlay the first chained
// payload header, where Offset represents Length.
//
// See MS-SMB2 2.2.42 for more details.
//
pub const SMB2_COMPRESSION_FLAG_NONE: c_uint = 0x0000;
pub const SMB2_COMPRESSION_FLAG_CHAINED: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_compression_hdr {
    pub /: *mut *mut __le32 ProtocolId; / 0xFC 'S' 'M' 'B',
    pub OriginalCompressedSegmentSize: __le32,
    pub CompressionAlgorithm: __le16,
    pub Flags: __le16,
    pub /: *mut *mut __le32 Offset; / this is the size of the uncompressed SMB2 header below,
// uncompressed SMB2 header (READ or WRITE) goes here
// compressed data goes here
    pub __packed: },
//
// ... OTOH, set compression payload header to always have OriginalPayloadSize
// as it's easier to pass the struct size minus sizeof(OriginalPayloadSize)
// than to juggle around the header/data memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_compression_payload_hdr {
    pub CompressionAlgorithm: __le16,
    pub Flags: __le16,
    pub /: *mut *mut __le32 Length; / length of compressed playload including field below if present,
    pub /: *mut *mut __le32 OriginalPayloadSize; / accounted when LZNT1, LZ77, LZ77+Huffman,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_compression_pattern_v1 {
    pub Pattern: __u8,
    pub Reserved1: __u8,
    pub Reserved2: __le16,
    pub Repetitions: __le32,
    pub __packed: },
// See MS-SMB2 section 2.2.9.2
// Context Types
pub const SMB2_RESERVED_TREE_CONNECT_CONTEXT_ID: c_uint = 0x0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tree_connect_contexts {
    pub ContextType: __le16,
    pub DataLength: __le16,
    pub Reserved: __le32,
    pub Data: [__u8; ],
    pub __packed: },
// Remoted identity tree connect context structures - see MS-SMB2 2.2.9.2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_blob_data {
    pub BlobSize: __le16,
    pub BlobData: [__u8; ],
    pub __packed: },
// Valid values for Attr
pub const SE_GROUP_MANDATORY: c_uint = 0x00000001;
pub const SE_GROUP_ENABLED_BY_DEFAULT: c_uint = 0x00000002;
pub const SE_GROUP_ENABLED: c_uint = 0x00000004;
pub const SE_GROUP_OWNER: c_uint = 0x00000008;
pub const SE_GROUP_USE_FOR_DENY_ONLY: c_uint = 0x00000010;
pub const SE_GROUP_INTEGRITY: c_uint = 0x00000020;
pub const SE_GROUP_INTEGRITY_ENABLED: c_uint = 0x00000040;
pub const SE_GROUP_RESOURCE: c_uint = 0x20000000;
pub const SE_GROUP_LOGON_ID: c_uint = 0xC0000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sid_array_data {
    pub SidAttrCount: __le16,
// SidAttrList - array of sid_attr_data structs
    pub __packed: },
// struct sid_attr_data is SidData array in BlobData format then le32 Attr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sid_attr_data {
    pub BlobSize: __le16,
    pub BlobData: [__u8; ],
// __le32 Attr
    pub __packed: },
//
// struct privilege_data is the same as BLOB_DATA - see MS-SMB2 2.2.9.2.1.5
// but with size of LUID_ATTR_DATA struct and BlobData set to LUID_ATTR DATA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct privilege_array_data {
    pub PrivilegeCount: __le16,
// array of privilege_data structs
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct remoted_identity_tcon_context {
    pub /: *mut *mut __le16 TicketType; / must be 0x0001,
    pub /: *mut *mut __le16 TicketSize; / total size of this struct,
    pub /: *mut *mut __le16 User; / offset to SID_ATTR_DATA struct with user info,
    pub /: *mut *mut __le16 UserName; / offset to null terminated Unicode username string,
    pub /: *mut *mut __le16 Domain; / offset to null terminated Unicode domain name,
    pub /: *mut *mut __le16 Groups; / offset to SID_ARRAY_DATA struct with group info,
    pub /: *mut *mut __le16 RestrictedGroups; / similar to above,
    pub /: *mut *mut __le16 Privileges; / offset to PRIVILEGE_ARRAY_DATA struct,
    pub /: *mut *mut __le16 PrimaryGroup; / offset to SID_ARRAY_DATA struct,
    pub /: *mut *mut __le16 Owner; / offset to BLOB_DATA struct,
    pub /: *mut *mut __le16 DefaultDacl; / offset to BLOB_DATA struct,
    pub /: *mut *mut __le16 DeviceGroups; / offset to SID_ARRAY_DATA struct,
    pub /: *mut *mut __le16 UserClaims; / offset to BLOB_DATA struct,
    pub /: *mut *mut __le16 DeviceClaims; / offset to BLOB_DATA struct,
    pub /: *mut *mut __u8 TicketInfo[]; / variable length buf - remoted identity data,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_tree_connect_req_extension {
    pub TreeConnectContextOffset: __le32,
    pub TreeConnectContextCount: __le16,
    pub Reserved: [__u8; 10],
    pub /: *mut *mut __u8 PathName[]; / variable sized array,
// followed by array of TreeConnectContexts
    pub __packed: },
// Flags/Reserved for SMB3.1.1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_tree_connect_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 9,
    pub /: *mut *mut __le16 Flags; / Flags in SMB3.1.1,
    pub PathOffset: __le16,
    pub PathLength: __le16,
    pub /: *mut *mut __u8 Buffer[]; / variable length,
    pub __packed: },
// Possible ShareType values
pub const SMB2_SHARE_TYPE_DISK: c_uint = 0x01;
pub const SMB2_SHARE_TYPE_PIPE: c_uint = 0x02;
pub const SMB2_SHARE_TYPE_PRINT: c_uint = 0x03;
//
// Possible ShareFlags - exactly one and only one of the first 4 caching flags
// must be set (any of the remaining, SHI1005, flags may be set individually
// or in combination.
//
pub const SMB2_SHAREFLAG_MANUAL_CACHING: c_uint = 0x00000000;
pub const SMB2_SHAREFLAG_AUTO_CACHING: c_uint = 0x00000010;
pub const SMB2_SHAREFLAG_VDO_CACHING: c_uint = 0x00000020;
pub const SMB2_SHAREFLAG_NO_CACHING: c_uint = 0x00000030;
pub const SHI1005_FLAGS_DFS: c_uint = 0x00000001;
pub const SHI1005_FLAGS_DFS_ROOT: c_uint = 0x00000002;
pub const SMB2_SHAREFLAG_RESTRICT_EXCLUSIVE_OPENS: c_uint = 0x00000100;
pub const SMB2_SHAREFLAG_FORCE_SHARED_DELETE: c_uint = 0x00000200;
pub const SMB2_SHAREFLAG_ALLOW_NAMESPACE_CACHING: c_uint = 0x00000400;
pub const SMB2_SHAREFLAG_ACCESS_BASED_DIRECTORY_ENUM: c_uint = 0x00000800;
pub const SMB2_SHAREFLAG_FORCE_LEVELII_OPLOCK: c_uint = 0x00001000;
pub const SMB2_SHAREFLAG_ENABLE_HASH_V1: c_uint = 0x00002000;
pub const SMB2_SHAREFLAG_ENABLE_HASH_V2: c_uint = 0x00004000;
pub const SMB2_SHAREFLAG_ENCRYPT_DATA: c_uint = 0x00008000;

pub const SMB2_SHAREFLAG_IDENTITY_REMOTING: c_uint = 0x00040000 /* 3.1.1 */;
pub const SMB2_SHAREFLAG_COMPRESS_DATA: c_uint = 0x00100000 /* 3.1.1 */;
pub const SMB2_SHAREFLAG_ISOLATED_TRANSPORT: c_uint = 0x00200000;
pub const SHI1005_FLAGS_ALL: c_uint = 0x0034FF33;
// Possible share capabilities

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_tree_connect_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 16,
    pub /: *mut *mut __u8 ShareType; / see below,
    pub Reserved: __u8,
    pub /: *mut *mut __le32 ShareFlags; / see below,
    pub /: *mut *mut __le32 Capabilities; / see below,
    pub MaximalAccess: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_tree_disconnect_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 4,
    pub Reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_tree_disconnect_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 4,
    pub Reserved: __le16,
    pub __packed: },
//
// SMB2_NEGOTIATE_PROTOCOL  See MS-SMB2 section 2.2.3
//
// SecurityMode flags
pub const SMB2_NEGOTIATE_SIGNING_ENABLED: c_uint = 0x0001;

pub const SMB2_NEGOTIATE_SIGNING_REQUIRED: c_uint = 0x0002;

pub const SMB2_SEC_MODE_FLAGS_ALL: c_uint = 0x0003;
// Capabilities flags
pub const SMB2_GLOBAL_CAP_DFS: c_uint = 0x00000001;
pub const SMB2_GLOBAL_CAP_LEASING: c_uint = 0x00000002 /* Resp only New to SMB2.1 */;
pub const SMB2_GLOBAL_CAP_LARGE_MTU: c_uint = 0x00000004 /* Resp only New to SMB2.1 */;
pub const SMB2_GLOBAL_CAP_MULTI_CHANNEL: c_uint = 0x00000008 /* New to SMB3 */;
pub const SMB2_GLOBAL_CAP_PERSISTENT_HANDLES: c_uint = 0x00000010 /* New to SMB3 */;
pub const SMB2_GLOBAL_CAP_DIRECTORY_LEASING: c_uint = 0x00000020 /* New to SMB3 */;
pub const SMB2_GLOBAL_CAP_ENCRYPTION: c_uint = 0x00000040 /* New to SMB3 */;
pub const SMB2_GLOBAL_CAP_NOTIFICATIONS: c_uint = 0x00000080 /* New to SMB3.1.1 */;
// Internal types
pub const SMB2_NT_FIND: c_uint = 0x00100000;
pub const SMB2_LARGE_FILES: c_uint = 0x00200000;
pub const SMB2_CLIENT_GUID_SIZE: c_int = 16;
pub const SMB2_CREATE_GUID_SIZE: c_int = 16;
// Dialects
pub const SMB10_PROT_ID: c_uint = 0x0000 /* local only, not sent on wire w/CIFS negprot */;
pub const SMB20_PROT_ID: c_uint = 0x0202;
pub const SMB21_PROT_ID: c_uint = 0x0210;
pub const SMB2X_PROT_ID: c_uint = 0x02FF;
pub const SMB30_PROT_ID: c_uint = 0x0300;
pub const SMB302_PROT_ID: c_uint = 0x0302;
pub const SMB311_PROT_ID: c_uint = 0x0311;
pub const BAD_PROT_ID: c_uint = 0xFFFF;
pub const SMB311_SALT_SIZE: c_int = 32;
// Hash Algorithm Types

pub const SMB2_PREAUTH_HASH_SIZE: c_int = 64;
// Negotiate Contexts - ContextTypes. See MS-SMB2 section 2.2.3.1 for details

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_neg_context {
    pub ContextType: __le16,
    pub DataLength: __le16,
    pub Reserved: __le32,
// Followed by array of data. NOTE: some servers require padding to 8 byte boundary
    pub __packed: },
//
// SaltLength that the server send can be zero, so the only three required
// fields (all __le16) end up six bytes total, so the minimum context data len
// in the response is six bytes which accounts for
//
// HashAlgorithmCount, SaltLength, and 1 HashAlgorithm.
//
pub const MIN_PREAUTH_CTXT_DATA_LEN: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_preauth_neg_context {
    pub /: *mut *mut __le16 ContextType; / 1,
    pub DataLength: __le16,
    pub Reserved: __le32,
    pub /: *mut *mut __le16 HashAlgorithmCount; / 1,
    pub SaltLength: __le16,
    pub /: *mut *mut __le16 HashAlgorithms; / HashAlgorithms[0] since only one defined,
    pub Salt: [__u8; SMB311_SALT_SIZE],
    pub __packed: },
// Encryption Algorithms Ciphers

// Min encrypt context data is one cipher so 2 bytes + 2 byte count field
pub const MIN_ENCRYPT_CTXT_DATA_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_encryption_neg_context {
    pub /: *mut *mut __le16 ContextType; / 2,
    pub DataLength: __le16,
    pub Reserved: __le32,
// CipherCount usually 2, but can be 3 when AES256-GCM enabled
    pub /: *mut *mut __le16 CipherCount; / AES128-GCM and AES128-CCM by default,
    pub Ciphers: [__le16; ],
    pub __packed: },
// See MS-SMB2 2.2.3.1.3

// Pattern scanning algorithm See MS-SMB2 3.1.4.4.1

// Account for NONE for easier array indexing
pub const SMB3_COMPRESS_MAX_ALGS: c_int = 6;
// Compression Flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_compression_capabilities_context {
    pub /: *mut *mut __le16 ContextType; / 3,
    pub DataLength: __le16,
    pub Reserved: __le32,
    pub CompressionAlgorithmCount: __le16,
    pub Padding: __le16,
    pub Flags: __le32,
    pub CompressionAlgorithms: [__le16; 4],
    pub __packed: },
//
// For smb2_netname_negotiate_context_id See MS-SMB2 2.2.3.1.4.
// Its struct simply contains NetName, an array of Unicode characters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_netname_neg_context {
    pub /: *mut *mut __le16 ContextType; / 5,
    pub DataLength: __le16,
    pub Reserved: __le32,
    pub /: *mut *mut __le16 NetName[]; / hostname of target converted to UCS-2,
    pub __packed: },
//
// For smb2_transport_capabilities context see MS-SMB2 2.2.3.1.5
// and 2.2.4.1.5
//
// Flags
pub const SMB2_ACCEPT_TRANSPORT_LEVEL_SECURITY: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_transport_capabilities_context {
    pub /: *mut *mut __le16 ContextType; / 6,
    pub DataLength: __le16,
    pub Reserved: __u32,
    pub Flags: __le32,
    pub Pad: __u32,
    pub __packed: },
//
// For rdma transform capabilities context see MS-SMB2 2.2.3.1.6
// and 2.2.4.1.6
//
// RDMA Transform IDs
pub const SMB2_RDMA_TRANSFORM_NONE: c_uint = 0x0000;
pub const SMB2_RDMA_TRANSFORM_ENCRYPTION: c_uint = 0x0001;
pub const SMB2_RDMA_TRANSFORM_SIGNING: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_rdma_transform_capabilities_context {
    pub /: *mut *mut __le16 ContextType; / 7,
    pub DataLength: __le16,
    pub Reserved: __u32,
    pub TransformCount: __le16,
    pub Reserved1: __u16,
    pub Reserved2: __u32,
    pub RDMATransformIds: [__le16; ],
    pub __packed: },
//
// For signing capabilities context see MS-SMB2 2.2.3.1.7
// and 2.2.4.1.7
//
// Signing algorithms
pub const SIGNING_ALG_HMAC_SHA256: c_int = 0;

pub const SIGNING_ALG_AES_CMAC: c_int = 1;

pub const SIGNING_ALG_AES_GMAC: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_signing_capabilities {
    pub /: *mut *mut __le16 ContextType; / 8,
    pub DataLength: __le16,
    pub Reserved: __le32,
    pub SigningAlgorithmCount: __le16,
    pub SigningAlgorithms: [__le16; ],
// Followed by padding to 8 byte boundary (required by some servers)
    pub __packed: },
pub const POSIX_CTXT_DATA_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_posix_neg_context {
    pub /: *mut *mut __le16 ContextType; / 0x100,
    pub DataLength: __le16,
    pub Reserved: __le32,
    pub /: *mut *mut __u8 Name[16]; / POSIX ctxt GUID 93AD25509CB411E7B42383DE968BCD7C,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_negotiate_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 36,
    pub DialectCount: __le16,
    pub SecurityMode: __le16,
    pub /: *mut *mut __le16 Reserved; / MBZ,
    pub Capabilities: __le32,
    pub ClientGUID: [__u8; SMB2_CLIENT_GUID_SIZE],
// In SMB3.02 and earlier next three were MBZ le64 ClientStartTime
    pub /: *mut *mut __le32 NegotiateContextOffset; / SMB3.1.1 only. MBZ earlier,
    pub /: *mut *mut __le16 NegotiateContextCount; / SMB3.1.1 only. MBZ earlier,
    pub Reserved2: __le16,
    pub Dialects: [__le16; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_negotiate_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 65,
    pub SecurityMode: __le16,
    pub DialectRevision: __le16,
    pub /: *mut *mut __le16 NegotiateContextCount; / Prior to SMB3.1.1 was Reserved & MBZ,
    pub ServerGUID: [__u8; 16],
    pub Capabilities: __le32,
    pub MaxTransactSize: __le32,
    pub MaxReadSize: __le32,
    pub MaxWriteSize: __le32,
    pub /: *mut *mut __le64 SystemTime; / MBZ,
    pub ServerStartTime: __le64,
    pub SecurityBufferOffset: __le16,
    pub SecurityBufferLength: __le16,
    pub /: *mut *mut __le32 NegotiateContextOffset; / Pre:SMB3.1.1 was reserved/ignored,
    pub /: *mut *mut __u8 Buffer[]; / variable length GSS security buffer,
    pub __packed: },
//
// SMB2_SESSION_SETUP  See MS-SMB2 section 2.2.5
//
// Flags
pub const SMB2_SESSION_REQ_FLAG_BINDING: c_uint = 0x01;
pub const SMB2_SESSION_REQ_FLAG_ENCRYPT_DATA: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_sess_setup_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 25,
    pub Flags: __u8,
    pub SecurityMode: __u8,
    pub Capabilities: __le32,
    pub Channel: __le32,
    pub SecurityBufferOffset: __le16,
    pub SecurityBufferLength: __le16,
    pub PreviousSessionId: __le64,
    pub /: *mut *mut __u8 Buffer[]; / variable length GSS security buffer,
    pub __packed: },
// Currently defined SessionFlags
pub const SMB2_SESSION_FLAG_IS_GUEST: c_uint = 0x0001;

pub const SMB2_SESSION_FLAG_IS_NULL: c_uint = 0x0002;

pub const SMB2_SESSION_FLAG_ENCRYPT_DATA: c_uint = 0x0004;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_sess_setup_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 9,
    pub SessionFlags: __le16,
    pub SecurityBufferOffset: __le16,
    pub SecurityBufferLength: __le16,
    pub /: *mut *mut __u8 Buffer[]; / variable length GSS security buffer,
    pub __packed: },
//
// SMB2_LOGOFF  See MS-SMB2 section 2.2.7
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_logoff_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 4,
    pub Reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_logoff_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 4,
    pub Reserved: __le16,
    pub __packed: },
//
// SMB2_CLOSE  See MS-SMB2 section 2.2.15
//
// Currently defined values for close flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_close_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 24,
    pub Flags: __le16,
    pub Reserved: __le32,
    pub /: *mut *mut __u64 PersistentFileId; / opaque endianness,
    pub /: *mut *mut __u64 VolatileFileId; / opaque endianness,
    pub __packed: },
//
// Maximum size of a SMB2_CLOSE response is 64 (smb2 header) + 60 (data)
//
pub const MAX_SMB2_CLOSE_RESPONSE_SIZE: c_int = 124;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_close_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / 60,
    pub Flags: __le16,
    pub Reserved: __le32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
// Beginning of FILE_STANDARD_INFO equivalent
    pub AllocationSize: __le64,
    pub EndOfFile: __le64,
    pub Attributes: __le32,
    pub __packed: },
//
// SMB2_READ  See MS-SMB2 section 2.2.19
//
// For read request Flags field below, following flag is defined for SMB3.02
pub const SMB2_READFLAG_READ_UNBUFFERED: c_uint = 0x01;
pub const SMB2_READFLAG_REQUEST_COMPRESSED: c_uint = 0x02 /* See MS-SMB2 2.2.19 */;
// Channel field for read and write: exactly one of following flags can be set

// See MS-SMB2 2.2.43.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_rdma_transform {
    pub RdmaDescriptorOffset: __le16,
    pub RdmaDescriptorLength: __le16,
    pub Channel: __le32,
    pub TransformCount: __le16,
    pub Reserved1: __le16,
    pub Reserved2: __le32,
    pub __packed: },
pub const SMB2_RDMA_TRANSFORM_TYPE_ENCRYPTION: c_uint = 0x0001;
pub const SMB2_RDMA_TRANSFORM_TYPE_SIGNING: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_rdma_crypto_transform {
    pub TransformType: __le16,
    pub SignatureLength: __le16,
    pub NonceLength: __le16,
    pub Reserved: __le16,
    pub Signature: [__u8; ],
// Followed by Nonce[] and optional alignment padding.
    pub __packed: },
// SMB2 read request without RFC1001 length at the beginning
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_read_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 49,
    pub /: *mut *mut __u8 Padding; / offset from start of SMB2 header to place read,
    pub /: *mut *mut __u8 Flags; / MBZ unless SMB3.02 or later,
    pub Length: __le32,
    pub Offset: __le64,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub MinimumCount: __le32,
    pub /: *mut *mut __le32 Channel; / MBZ except for SMB3 or later,
    pub RemainingBytes: __le32,
    pub ReadChannelInfoOffset: __le16,
    pub ReadChannelInfoLength: __le16,
    pub Buffer: [__u8; ],
    pub __packed: },
// Read flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_read_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 17,
    pub DataOffset: __u8,
    pub Reserved: __u8,
    pub DataLength: __le32,
    pub DataRemaining: __le32,
    pub Flags: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
//
// SMB2_WRITE  See MS-SMB2 section 2.2.21
//
// For write request Flags field below the following flags are defined:
pub const SMB2_WRITEFLAG_WRITE_THROUGH: c_uint = 0x00000001	/* SMB2.1 or later */;
pub const SMB2_WRITEFLAG_WRITE_UNBUFFERED: c_uint = 0x00000002	/* SMB3.02 or later */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_write_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 49,
    pub /: *mut *mut __le16 DataOffset; / offset from start of SMB2 header to write data,
    pub Length: __le32,
    pub Offset: __le64,
    pub /: *mut *mut __u64 PersistentFileId; / opaque endianness,
    pub /: *mut *mut __u64 VolatileFileId; / opaque endianness,
    pub /: *mut *mut __le32 Channel; / MBZ unless SMB3.02 or later,
    pub RemainingBytes: __le32,
    pub WriteChannelInfoOffset: __le16,
    pub WriteChannelInfoLength: __le16,
    pub Flags: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_write_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 17,
    pub DataOffset: __u8,
    pub Reserved: __u8,
    pub DataLength: __le32,
    pub DataRemaining: __le32,
    pub Reserved2: __u32,
    pub Buffer: [__u8; ],
    pub __packed: },
//
// SMB2_FLUSH  See MS-SMB2 section 2.2.17
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_flush_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 24,
    pub Reserved1: __le16,
    pub Reserved2: __le32,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_flush_rsp {
    pub hdr: smb2_hdr,
    pub StructureSize: __le16,
    pub Reserved: __le16,
    pub __packed: },
pub const SMB2_LOCKFLAG_SHARED: c_uint = 0x0001;
pub const SMB2_LOCKFLAG_EXCLUSIVE: c_uint = 0x0002;
pub const SMB2_LOCKFLAG_UNLOCK: c_uint = 0x0004;
pub const SMB2_LOCKFLAG_FAIL_IMMEDIATELY: c_uint = 0x0010;
pub const SMB2_LOCKFLAG_MASK: c_uint = 0x0007;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_lock_element {
    pub Offset: __le64,
    pub Length: __le64,
    pub Flags: __le32,
    pub Reserved: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_lock_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 48,
    pub LockCount: __le16,
//
// The least significant four bits are the lock sequence number. The
// other 28 bits are the index (0 to 64). See MS-SMB2 2.2.26.
//
    pub LockSequenceNumber: __le32,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
// Followed by at least one
    pub lock: smb2_lock_element,
    pub locks): DECLARE_FLEX_ARRAY(struct smb2_lock_element,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_lock_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 4,
    pub Reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_echo_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 4,
    pub Reserved: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_echo_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 4,
    pub Reserved: __u16,
    pub __packed: },
//
// Valid FileInformation classes for query directory
//
// Note that these are a subset of the (file) QUERY_INFO levels defined
// later in this file (but since QUERY_DIRECTORY uses equivalent numbers
// we do not redefine them here)
//
// FileDirectoryInfomation		0x01
// FileFullDirectoryInformation		0x02
// FileIdFullDirectoryInformation	0x26
// FileBothDirectoryInformation		0x03
// FileIdBothDirectoryInformation	0x25
// FileNamesInformation			0x0C
// FileIdExtdDirectoryInformation	0x3C
//
// search (query_directory) Flags field
pub const SMB2_RESTART_SCANS: c_uint = 0x01;
pub const SMB2_RETURN_SINGLE_ENTRY: c_uint = 0x02;
pub const SMB2_INDEX_SPECIFIED: c_uint = 0x04;
pub const SMB2_REOPEN: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_query_directory_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 33,
    pub FileInformationClass: __u8,
    pub Flags: __u8,
    pub FileIndex: __le32,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub FileNameOffset: __le16,
    pub FileNameLength: __le16,
    pub OutputBufferLength: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_query_directory_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 9,
    pub OutputBufferOffset: __le16,
    pub OutputBufferLength: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
// DeviceType Flags
pub const FILE_DEVICE_CD_ROM: c_uint = 0x00000002;
pub const FILE_DEVICE_CD_ROM_FILE_SYSTEM: c_uint = 0x00000003;
pub const FILE_DEVICE_DFS: c_uint = 0x00000006;
pub const FILE_DEVICE_DISK: c_uint = 0x00000007;
pub const FILE_DEVICE_DISK_FILE_SYSTEM: c_uint = 0x00000008;
pub const FILE_DEVICE_FILE_SYSTEM: c_uint = 0x00000009;
pub const FILE_DEVICE_NAMED_PIPE: c_uint = 0x00000011;
pub const FILE_DEVICE_NETWORK: c_uint = 0x00000012;
pub const FILE_DEVICE_NETWORK_FILE_SYSTEM: c_uint = 0x00000014;
pub const FILE_DEVICE_NULL: c_uint = 0x00000015;
pub const FILE_DEVICE_PARALLEL_PORT: c_uint = 0x00000016;
pub const FILE_DEVICE_PRINTER: c_uint = 0x00000018;
pub const FILE_DEVICE_SERIAL_PORT: c_uint = 0x0000001b;
pub const FILE_DEVICE_STREAMS: c_uint = 0x0000001e;
pub const FILE_DEVICE_TAPE: c_uint = 0x0000001f;
pub const FILE_DEVICE_TAPE_FILE_SYSTEM: c_uint = 0x00000020;
pub const FILE_DEVICE_VIRTUAL_DISK: c_uint = 0x00000024;
pub const FILE_DEVICE_NETWORK_REDIRECTOR: c_uint = 0x00000028;
// Device Characteristics
pub const FILE_REMOVABLE_MEDIA: c_uint = 0x00000001;
pub const FILE_READ_ONLY_DEVICE: c_uint = 0x00000002;
pub const FILE_FLOPPY_DISKETTE: c_uint = 0x00000004;
pub const FILE_WRITE_ONCE_MEDIA: c_uint = 0x00000008;
pub const FILE_REMOTE_DEVICE: c_uint = 0x00000010;
pub const FILE_DEVICE_IS_MOUNTED: c_uint = 0x00000020;
pub const FILE_VIRTUAL_VOLUME: c_uint = 0x00000040;
pub const FILE_DEVICE_SECURE_OPEN: c_uint = 0x00000100;
pub const FILE_CHARACTERISTIC_TS_DEVICE: c_uint = 0x00001000;
pub const FILE_CHARACTERISTIC_WEBDAV_DEVICE: c_uint = 0x00002000;
pub const FILE_PORTABLE_DEVICE: c_uint = 0x00004000;
pub const FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL: c_uint = 0x00020000;
//
// Maximum number of iovs we need for a set-info request.
// The largest one is rename/hardlink
// [0] : struct smb2_set_info_req + smb2_file_[rename|link]_info
// [1] : path
// [2] : compound padding
//
pub const SMB2_SET_INFO_IOV_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_set_info_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 33,
    pub InfoType: __u8,
    pub FileInfoClass: __u8,
    pub BufferLength: __le32,
    pub BufferOffset: __le16,
    pub Reserved: __u16,
    pub AdditionalInformation: __le32,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub Buffer: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_set_info_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 2,
    pub __packed: },
//
// SMB2_NOTIFY  See MS-SMB2 section 2.2.35
//
// notify flags
pub const SMB2_WATCH_TREE: c_uint = 0x0001;
// notify completion filter flags. See MS-FSCC 2.6 and MS-SMB2 2.2.35
pub const FILE_NOTIFY_CHANGE_FILE_NAME: c_uint = 0x00000001;
pub const FILE_NOTIFY_CHANGE_DIR_NAME: c_uint = 0x00000002;
pub const FILE_NOTIFY_CHANGE_NAME: c_uint = 0x00000003;
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: c_uint = 0x00000004;
pub const FILE_NOTIFY_CHANGE_SIZE: c_uint = 0x00000008;
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: c_uint = 0x00000010;
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: c_uint = 0x00000020;
pub const FILE_NOTIFY_CHANGE_CREATION: c_uint = 0x00000040;
pub const FILE_NOTIFY_CHANGE_EA: c_uint = 0x00000080;
pub const FILE_NOTIFY_CHANGE_SECURITY: c_uint = 0x00000100;
pub const FILE_NOTIFY_CHANGE_STREAM_NAME: c_uint = 0x00000200;
pub const FILE_NOTIFY_CHANGE_STREAM_SIZE: c_uint = 0x00000400;
pub const FILE_NOTIFY_CHANGE_STREAM_WRITE: c_uint = 0x00000800;
// See MS-SMB2 2.2.35
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_change_notify_req {
    pub hdr: smb2_hdr,
    pub StructureSize: __le16,
    pub Flags: __le16,
    pub OutputBufferLength: __le32,
    pub /: *mut *mut __u64 PersistentFileId; / opaque endianness,
    pub /: *mut *mut __u64 VolatileFileId; / opaque endianness,
    pub CompletionFilter: __le32,
    pub Reserved: __u32,
    pub __packed: },
// See MS-SMB2 2.2.36
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_change_notify_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 9,
    pub OutputBufferOffset: __le16,
    pub OutputBufferLength: __le32,
    pub /: *mut *mut __u8 Buffer[]; / array of file notify structs,
    pub __packed: },
//
// SMB2_SERVER_TO_CLIENT_NOTIFICATION: See MS-SMB2 section 2.2.44
//
pub const SMB2_NOTIFY_SESSION_CLOSED: c_uint = 0x0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_server_client_notification {
    pub hdr: smb2_hdr,
    pub StructureSize: __le16,
    pub /: *mut *mut __u16 Reserved; / MBZ,
    pub NotificationType: __le32,
    pub /: *mut *mut __u8 NotificationBuffer[4]; / MBZ,
    pub __packed: },
//
// SMB2_CREATE  See MS-SMB2 section 2.2.13
//
// Oplock levels
pub const SMB2_OPLOCK_LEVEL_NONE: c_uint = 0x00;
pub const SMB2_OPLOCK_LEVEL_II: c_uint = 0x01;
pub const SMB2_OPLOCK_LEVEL_EXCLUSIVE: c_uint = 0x08;
pub const SMB2_OPLOCK_LEVEL_BATCH: c_uint = 0x09;
pub const SMB2_OPLOCK_LEVEL_LEASE: c_uint = 0xFF;
// Non-spec internal type
pub const SMB2_OPLOCK_LEVEL_NOCHANGE: c_uint = 0x99;
// Impersonation Levels. See MS-WPO section 9.7 and MSDN-IMPERS

// Desired Access Flags

// ShareAccess Flags

// CreateDisposition Flags

// CreateOptions Flags

// same as #define CREATE_NOT_FILE_LE	cpu_to_le32(0x00000001)

// FILE_SYNCHRONOUS_IO_ALERT_LE		cpu_to_le32(0x00000010) should be zero, ignored
// FILE_SYNCHRONOUS_IO_NONALERT		cpu_to_le32(0x00000020) should be zero, ignored

// FILE_OPEN_REMOTE_INSTANCE		cpu_to_le32(0x00000400) should be zero, ignored

// FILE_OPEN_REQUIRING_OPLOCK		cpu_to_le32(0x00010000) should be zero, ignored
// FILE_DISALLOW_EXCLUSIVE		cpu_to_le32(0x00020000) should be zero, ignored
// FILE_RESERVE_OPFILTER		cpu_to_le32(0x00100000) MBZ

// #define FILE_OPEN_FOR_FREE_SPACE_QUERY cpu_to_le32(0x00800000) should be zero, ignored

// Create Context Values

// Flag (SMB3 open response) values
pub const SMB2_CREATE_FLAG_REPARSEPOINT: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_context {
// New members must be added within the struct_group() macro below.
    pub Next: __le32,
    pub NameOffset: __le16,
    pub NameLength: __le16,
    pub Reserved: __le16,
    pub DataOffset: __le16,
    pub DataLength: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_create_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 57,
    pub SecurityFlags: __u8,
    pub RequestedOplockLevel: __u8,
    pub ImpersonationLevel: __le32,
    pub SmbCreateFlags: __le64,
    pub Reserved: __le64,
    pub DesiredAccess: __le32,
    pub FileAttributes: __le32,
    pub ShareAccess: __le32,
    pub CreateDisposition: __le32,
    pub CreateOptions: __le32,
    pub NameOffset: __le16,
    pub NameLength: __le16,
    pub CreateContextsOffset: __le32,
    pub CreateContextsLength: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_create_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 89,
    pub OplockLevel: __u8,
    pub /: *mut *mut __u8 Flags; / 0x01 if reparse point,
    pub CreateAction: __le32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub AllocationSize: __le64,
    pub EndofFile: __le64,
    pub FileAttributes: __le32,
    pub Reserved2: __le32,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub CreateContextsOffset: __le32,
    pub CreateContextsLength: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_posix {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 16],
    pub Mode: __le32,
    pub Reserved: __u32,
    pub __packed: },
// See MS-SMB2 2.2.13.2.3 and MS-SMB2 2.2.13.2.4
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub Reserved: [__u8; 16],
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub Fid: },
    pub Data: },
    pub create_durable_reconn_t: } __packed create_durable_req_t,,
// See MS-SMB2 2.2.13.2.5
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_mxac_req {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub Timestamp: __le64,
    pub __packed: },
//
// AAPL flags. See Samba libcli/smb/smb2_create_ctx.h
//
// "AAPL" Context Command Codes
pub const SMB2_CRTCTX_AAPL_SERVER_QUERY: c_int = 1;
pub const SMB2_CRTCTX_AAPL_RESOLVE_ID: c_int = 2;
// "AAPL" Server Query request/response bitmap
pub const SMB2_CRTCTX_AAPL_SERVER_CAPS: c_int = 1;
pub const SMB2_CRTCTX_AAPL_VOLUME_CAPS: c_int = 2;
pub const SMB2_CRTCTX_AAPL_MODEL_INFO: c_int = 4;
// "AAPL" Client/Server Capabilities bitmap
pub const SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR: c_int = 1;
pub const SMB2_CRTCTX_AAPL_SUPPORTS_OSX_COPYFILE: c_int = 2;
pub const SMB2_CRTCTX_AAPL_UNIX_BASED: c_int = 4;
pub const SMB2_CRTCTX_AAPL_SUPPORTS_NFS_ACE: c_int = 8;
//
// V2 extends the same inline-FinderInfo mechanism as
// SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR with an added flags field,
// confirmed byte-identical to V1 otherwise against AAPL's actual
// public client behavior.  Mutually exclusive with the V1 bit on
// the wire, not both set together.
//
pub const SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR_V2: c_int = 16;
// "AAPL" Volume Capabilities bitmap
pub const SMB2_CRTCTX_AAPL_SUPPORT_RESOLVE_ID: c_int = 1;
pub const SMB2_CRTCTX_AAPL_CASE_SENSITIVE: c_int = 2;
pub const SMB2_CRTCTX_AAPL_FULL_SYNC: c_int = 4;
//
// Flags
// See MS-SMB2 2.2.13.2.11
// MS-SMB2 2.2.13.2.12
// MS-SMB2 2.2.14.2.12
//
pub const SMB2_DHANDLE_FLAG_PERSISTENT: c_uint = 0x00000002;
// See MS-SMB2 2.2.13.2.11
#[repr(C)]
#[derive(Copy, Clone)]
pub struct durable_context_v2_req {
    pub Timeout: __le32,
    pub /: *mut *mut __le32 Flags; / see SMB2_DHANDLE_FLAG_PERSISTENT,
    pub Reserved: __u64,
    pub CreateGuid: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_durable_req_v2 {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub dcontext: durable_context_v2_req,
    pub __packed: },
// See MS-SMB2 2.2.13.2.12
#[repr(C)]
#[derive(Copy, Clone)]
pub struct durable_reconnect_context_v2 {
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub Fid: },
    pub CreateGuid: [__u8; 16],
    pub /: *mut *mut __le32 Flags; / see SMB2_DHANDLE_FLAG_PERSISTENT,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_durable_handle_reconnect_v2 {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub dcontext: durable_reconnect_context_v2,
    pub Pad: [__u8; 4],
    pub __packed: },
// See MS-SMB2 2.2.14.2.12
#[repr(C)]
#[derive(Copy, Clone)]
pub struct durable_context_v2_rsp {
    pub Timeout: __le32,
    pub /: *mut *mut __le32 Flags; / see SMB2_DHANDLE_FLAG_PERSISTENT,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_durable_rsp_v2 {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub dcontext: durable_context_v2_rsp,
    pub __packed: },
// See MS-SMB2 2.2.14.2.5
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_mxac_rsp {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub QueryStatus: __le32,
    pub MaximalAccess: __le32,
    pub __packed: },

pub const SMB2_LEASE_KEY_SIZE: c_int = 16;
// See MS-SMB2 2.2.13.2.8
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lease_context {
    pub LeaseKey: [__u8; SMB2_LEASE_KEY_SIZE],
    pub LeaseState: __le32,
    pub LeaseFlags: __le32,
    pub LeaseDuration: __le64,
    pub __packed: },
// See MS-SMB2 2.2.13.2.10
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lease_context_v2 {
    pub LeaseKey: [__u8; SMB2_LEASE_KEY_SIZE],
    pub LeaseState: __le32,
    pub LeaseFlags: __le32,
    pub LeaseDuration: __le64,
    pub ParentLeaseKey: [__u8; SMB2_LEASE_KEY_SIZE],
    pub Epoch: __le16,
    pub Reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_lease {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub lcontext: lease_context,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_lease_v2 {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub lcontext: lease_context_v2,
    pub Pad: [__u8; 4],
    pub __packed: },
// See MS-SMB2 2.2.14.2.9
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_disk_id_rsp {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub DiskFileId: __le64,
    pub VolumeId: __le64,
    pub Reserved: [__u8; 16],
    pub __packed: },
// See MS-SMB2 2.2.13.2.13
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_app_inst_id {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 16],
    pub /: *mut *mut __le32 StructureSize; / Must be 20,
    pub Reserved: __u16,
    pub AppInstanceId: [__u8; 16],
    pub __packed: },
// See MS-SMB2 2.2.13.2.15
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_app_inst_id_vers {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 16],
    pub /: *mut *mut __le32 StructureSize; / Must be 24,
    pub Reserved: __u16,
    pub Padding: __u32,
    pub AppInstanceVersionHigh: __le64,
    pub AppInstanceVersionLow: __le64,
    pub __packed: },
// See MS-SMB2 2.2.31 and 2.2.32
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_ioctl_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 57,
    pub /: *mut *mut __le16 Reserved; / offset from start of SMB2 header to write data,
    pub CtlCode: __le32,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub /: *mut *mut __le32 InputOffset; / Reserved MBZ,
    pub InputCount: __le32,
    pub MaxInputResponse: __le32,
    pub OutputOffset: __le32,
    pub OutputCount: __le32,
    pub MaxOutputResponse: __le32,
    pub Flags: __le32,
    pub Reserved2: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
// See MS-SMB2 2.2.31.1.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srv_copychunk {
    pub SourceOffset: __le64,
    pub TargetOffset: __le64,
    pub Length: __le32,
    pub Reserved: __le32,
    pub __packed: },
pub const COPY_CHUNK_RES_KEY_SIZE: c_int = 24;
// See MS-SMB2 2.2.31.1
// this goes in the ioctl buffer when doing a copychunk request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct copychunk_ioctl_req {
    pub SourceKey: [c_char; COPY_CHUNK_RES_KEY_SIZE],
    pub SourceKeyU64: [__le64; 3],
}

// See MS-SMB2 2.2.32.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct copychunk_ioctl_rsp {
    pub ChunksWritten: __le32,
    pub ChunkBytesWritten: __le32,
    pub TotalBytesWritten: __le32,
    pub __packed: },
// See MS-SMB2 2.2.32.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resume_key_ioctl_rsp {
    pub ResumeKey: [c_char; COPY_CHUNK_RES_KEY_SIZE],
    pub ResumeKeyU64: [__u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_ioctl_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 49,
    pub Reserved: __le16,
    pub CtlCode: __le32,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub /: *mut *mut __le32 InputOffset; / Reserved MBZ,
    pub InputCount: __le32,
    pub OutputOffset: __le32,
    pub OutputCount: __le32,
    pub Flags: __le32,
    pub Reserved2: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
// See MS-SMB2 2.2.32.5.1.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_sockaddr_in {
    pub Port: __be16,
    pub IPv4Address: __be32,
    pub Reserved: [__u8; 8],
    pub __packed: },
// See MS-SMB2 2.2.32.5.1.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_sockaddr_in6 {
    pub Port: __be16,
    pub FlowInfo: __be32,
    pub IPv6Address: [__u8; 16],
    pub ScopeId: __be32,
    pub __packed: },
// See MS-SMB2 2.2.32.5 and MS-SMB2 2.2.32.5.1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct network_interface_info_ioctl_rsp {
    pub /: *mut *mut __le32 Next; / next interface. zero if this is last one,
    pub IfIndex: __le32,
    pub /: *mut *mut __le32 Capability; / RSS or RDMA Capable,
    pub Reserved: __le32,
    pub LinkSpeed: __le64,
    pub SockAddr_Storage: [c_char; 128],
    pub Family: __le16,
    pub Buffer: [__u8; 126],
}

// Integrity ChecksumAlgorithm choices for above
pub const CHECKSUM_TYPE_NONE: c_uint = 0x0000;
pub const CHECKSUM_TYPE_CRC64: c_uint = 0x0002;
pub const CHECKSUM_TYPE_UNCHANGED: c_uint = 0xFFFF	/* set only */;
// Integrity flags for above
pub const FSCTL_INTEGRITY_FLAG_CHECKSUM_ENFORCEMENT_OFF: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct validate_negotiate_info_req {
    pub Capabilities: __le32,
    pub Guid: [__u8; SMB2_CLIENT_GUID_SIZE],
    pub SecurityMode: __le16,
    pub DialectCount: __le16,
    pub /: *mut *mut __le16 Dialects[4]; / BB expand this if autonegotiate > 4 dialects,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct validate_negotiate_info_rsp {
    pub Capabilities: __le32,
    pub Guid: [__u8; SMB2_CLIENT_GUID_SIZE],
    pub SecurityMode: __le16,
    pub /: *mut *mut __le16 Dialect; / Dialect in use for the connection,
    pub __packed: },
// Possible InfoType values
pub const SMB2_O_INFO_FILE: c_uint = 0x01;
pub const SMB2_O_INFO_FILESYSTEM: c_uint = 0x02;
pub const SMB2_O_INFO_SECURITY: c_uint = 0x03;
pub const SMB2_O_INFO_QUOTA: c_uint = 0x04;
// SMB2 Query Info see MS-SMB2 (2.2.37) or MS-DTYP
// List of QUERY INFO levels (those also valid for QUERY_DIR are noted below

pub const FILE_BASIC_INFORMATION: c_int = 4;
pub const FILE_STANDARD_INFORMATION: c_int = 5;
pub const FILE_INTERNAL_INFORMATION: c_int = 6;
pub const FILE_EA_INFORMATION: c_int = 7;
pub const FILE_ACCESS_INFORMATION: c_int = 8;
pub const FILE_NAME_INFORMATION: c_int = 9;
pub const FILE_RENAME_INFORMATION: c_int = 10;
pub const FILE_LINK_INFORMATION: c_int = 11;

pub const FILE_DISPOSITION_INFORMATION: c_int = 13;
pub const FILE_POSITION_INFORMATION: c_int = 14;
pub const FILE_FULL_EA_INFORMATION: c_int = 15;
pub const FILE_MODE_INFORMATION: c_int = 16;
pub const FILE_ALIGNMENT_INFORMATION: c_int = 17;
pub const FILE_ALL_INFORMATION: c_int = 18;
pub const FILE_ALLOCATION_INFORMATION: c_int = 19;
pub const FILE_END_OF_FILE_INFORMATION: c_int = 20;
pub const FILE_ALTERNATE_NAME_INFORMATION: c_int = 21;
pub const FILE_STREAM_INFORMATION: c_int = 22;
pub const FILE_PIPE_INFORMATION: c_int = 23;
pub const FILE_PIPE_LOCAL_INFORMATION: c_int = 24;
pub const FILE_PIPE_REMOTE_INFORMATION: c_int = 25;
pub const FILE_MAILSLOT_QUERY_INFORMATION: c_int = 26;
pub const FILE_MAILSLOT_SET_INFORMATION: c_int = 27;
pub const FILE_COMPRESSION_INFORMATION: c_int = 28;
pub const FILE_OBJECT_ID_INFORMATION: c_int = 29;
// Number 30 not defined in documents
pub const FILE_MOVE_CLUSTER_INFORMATION: c_int = 31;
pub const FILE_QUOTA_INFORMATION: c_int = 32;
pub const FILE_REPARSE_POINT_INFORMATION: c_int = 33;
pub const FILE_NETWORK_OPEN_INFORMATION: c_int = 34;
pub const FILE_ATTRIBUTE_TAG_INFORMATION: c_int = 35;
pub const FILE_TRACKING_INFORMATION: c_int = 36;

pub const FILE_VALID_DATA_LENGTH_INFORMATION: c_int = 39;
pub const FILE_SHORT_NAME_INFORMATION: c_int = 40;
pub const FILE_SFIO_RESERVE_INFORMATION: c_int = 44;
pub const FILE_SFIO_VOLUME_INFORMATION: c_int = 45;
pub const FILE_HARD_LINK_INFORMATION: c_int = 46;
pub const FILE_NORMALIZED_NAME_INFORMATION: c_int = 48;
pub const FILEID_GLOBAL_TX_DIRECTORY_INFORMATION: c_int = 50;
pub const FILE_STANDARD_LINK_INFORMATION: c_int = 54;
pub const FILE_ID_INFORMATION: c_int = 59;

// Used for Query Info and Find File POSIX Info for SMB3.1.1 and SMB1
pub const SMB_FIND_FILE_POSIX_INFO: c_uint = 0x064;
// Security info type additionalinfo flags.
pub const OWNER_SECINFO: c_uint = 0x00000001;
pub const GROUP_SECINFO: c_uint = 0x00000002;
pub const DACL_SECINFO: c_uint = 0x00000004;
pub const SACL_SECINFO: c_uint = 0x00000008;
pub const LABEL_SECINFO: c_uint = 0x00000010;
pub const ATTRIBUTE_SECINFO: c_uint = 0x00000020;
pub const SCOPE_SECINFO: c_uint = 0x00000040;
pub const BACKUP_SECINFO: c_uint = 0x00010000;
pub const UNPROTECTED_SACL_SECINFO: c_uint = 0x10000000;
pub const UNPROTECTED_DACL_SECINFO: c_uint = 0x20000000;
pub const PROTECTED_SACL_SECINFO: c_uint = 0x40000000;
pub const PROTECTED_DACL_SECINFO: c_uint = 0x80000000;
// Flags used for FileFullEAinfo
pub const SL_RESTART_SCAN: c_uint = 0x00000001;
pub const SL_RETURN_SINGLE_ENTRY: c_uint = 0x00000002;
pub const SL_INDEX_SPECIFIED: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_query_info_req {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 41,
    pub InfoType: __u8,
    pub FileInfoClass: __u8,
    pub OutputBufferLength: __le32,
    pub InputBufferOffset: __le16,
    pub Reserved: __u16,
    pub InputBufferLength: __le32,
    pub AdditionalInformation: __le32,
    pub Flags: __le32,
    pub PersistentFileId: __u64,
    pub VolatileFileId: __u64,
    pub Buffer: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_query_info_rsp {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 9,
    pub OutputBufferOffset: __le16,
    pub OutputBufferLength: __le32,
    pub Buffer: [__u8; ],
    pub __packed: },
// Level 100 query info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb311_posix_qinfo {
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
    pub Sids: [u8; ],
//
// var sized owner SID
// var sized group SID
// le32 filenamelength
// u8  filename[]
//
    pub __packed: },
// See MS-SMB2 2.2.23 through 2.2.25
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_oplock_break {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 24,
    pub OplockLevel: __u8,
    pub Reserved: __u8,
    pub Reserved2: __le32,
    pub PersistentFid: __u64,
    pub VolatileFid: __u64,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_lease_break {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 44,
    pub Epoch: __le16,
    pub Flags: __le32,
    pub LeaseKey: [__u8; 16],
    pub CurrentLeaseState: __le32,
    pub NewLeaseState: __le32,
    pub BreakReason: __le32,
    pub AccessMaskHint: __le32,
    pub ShareMaskHint: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_lease_ack {
    pub hdr: smb2_hdr,
    pub /: *mut *mut __le16 StructureSize; / Must be 36,
    pub Reserved: __le16,
    pub Flags: __le32,
    pub LeaseKey: [__u8; 16],
    pub LeaseState: __le32,
    pub LeaseDuration: __le64,
    pub __packed: },
pub const OP_BREAK_STRUCT_SIZE_20: c_int = 24;
pub const OP_BREAK_STRUCT_SIZE_21: c_int = 36;
//
// See MS-SMB2 2.2.13.1.1
// MS-SMB 2.2.1.4.1
// These are the file access permission bits defined in CIFS for the
// NTCreateAndX as well as the level 0x107
// TRANS2_QUERY_PATH_INFORMATION API.  The level 0x107, SMB_QUERY_FILE_ALL_INFO
// responds with the AccessFlags.
// The AccessFlags specifies the access permissions a caller has to the
// file and can have any suitable combination of the following values:
//
pub const FILE_READ_DATA: c_uint = 0x00000001  /* Data can be read from the file   */;
// or directory child entries can
// be listed together with the
// associated child attributes
// (so the FILE_READ_ATTRIBUTES on
// the child entry is not needed)
pub const FILE_WRITE_DATA: c_uint = 0x00000002  /* Data can be written to the file  */;
// or new file can be created in
// the directory
pub const FILE_APPEND_DATA: c_uint = 0x00000004  /* Data can be appended to the file */;
// (for non-local files over SMB it
// is same as FILE_WRITE_DATA)
// or new subdirectory can be
// created in the directory
pub const FILE_READ_EA: c_uint = 0x00000008  /* Extended attributes associated   */;
// with the file can be read
pub const FILE_WRITE_EA: c_uint = 0x00000010  /* Extended attributes associated   */;
// with the file can be written
pub const FILE_EXECUTE: c_uint = 0x00000020  /*Data can be read into memory from */;
// the file using system paging I/O
// for executing the file / script
// or right to traverse directory
// (but by default all users have
// directory bypass traverse
// privilege and do not need this
// permission on directories at all)
pub const FILE_DELETE_CHILD: c_uint = 0x00000040  /* Child entry can be deleted from  */;
// the directory (so the DELETE on
// the child entry is not needed)
pub const FILE_READ_ATTRIBUTES: c_uint = 0x00000080  /* Attributes associated with the   */;
// file or directory can be read
pub const FILE_WRITE_ATTRIBUTES: c_uint = 0x00000100  /* Attributes associated with the   */;
// file or directory can be written
pub const DELETE: c_uint = 0x00010000  /* The file or dir can be deleted   */;
pub const READ_CONTROL: c_uint = 0x00020000  /* The discretionary access control */;
// list and ownership associated
// with the file or dir can be read
pub const WRITE_DAC: c_uint = 0x00040000  /* The discretionary access control */;
// list associated with the file or
// directory can be written
pub const WRITE_OWNER: c_uint = 0x00080000  /* Ownership information associated */;
// with the file/dir can be written
pub const SYNCHRONIZE: c_uint = 0x00100000  /* The file handle can waited on to */;
// synchronize with the completion
// of an input/output request
pub const SYSTEM_SECURITY: c_uint = 0x01000000  /* The system access control list   */;
// associated with the file or
// directory can be read or written
// (cannot be in DACL, can in SACL)
pub const MAXIMUM_ALLOWED: c_uint = 0x02000000  /* Maximal subset of GENERIC_ALL    */;
// permissions which can be granted
// (cannot be in DACL nor SACL)
pub const GENERIC_ALL: c_uint = 0x10000000  /* Same as: GENERIC_EXECUTE |       */;
// GENERIC_WRITE |
// GENERIC_READ |
// FILE_DELETE_CHILD |
// DELETE |
// WRITE_DAC |
// WRITE_OWNER
// So GENERIC_ALL contains all bits
// mentioned above except these two
// SYSTEM_SECURITY  MAXIMUM_ALLOWED
pub const GENERIC_EXECUTE: c_uint = 0x20000000  /* Same as: FILE_EXECUTE |          */;
// FILE_READ_ATTRIBUTES |
// READ_CONTROL |
// SYNCHRONIZE
pub const GENERIC_WRITE: c_uint = 0x40000000  /* Same as: FILE_WRITE_DATA |       */;
// FILE_APPEND_DATA |
// FILE_WRITE_EA |
// FILE_WRITE_ATTRIBUTES |
// READ_CONTROL |
// SYNCHRONIZE
pub const GENERIC_READ: c_uint = 0x80000000  /* Same as: FILE_READ_DATA |        */;
// FILE_READ_EA |
// FILE_READ_ATTRIBUTES |
// READ_CONTROL |
// SYNCHRONIZE
// Combinations of file access permission bits

