//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/cavium/nitrox/nitrox_req.h
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

pub const PENDING_SIG: c_uint = 0xFFFFFFFFFFFFFFFFUL;
pub const PRIO: c_int = 4001;
extern "C" {
    pub fn void(req: *mut *mut sereq_completion_t)(void, err: c_int) -> typedef;
}
//
// struct gphdr - General purpose Header
// @param0: first parameter.
// @param1: second parameter.
// @param2: third parameter.
// @param3: fourth parameter.
//
// Params tell the iv and enc/dec data offsets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gphdr {
    pub param0: __be16,
    pub param1: __be16,
    pub param2: __be16,
    pub param3: __be16,
}

//
// struct se_req_ctrl - SE request information.
// @arg: Minor number of the opcode
// @ctxc: Context control.
// @unca: Uncertainity enabled.
// @info: Additional information for SE cores.
// @ctxl: Context length in bytes.
// @uddl: User defined data length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union se_req_ctrl {
    pub value: u64,
    pub 22: u64 raz :,
    pub 8: u64 arg :,
    pub 2: u64 ctxc :,
    pub 1: u64 unca :,
    pub 3: u64 info :,
    pub 8: u64 unc :,
    pub 12: u64 ctxl :,
    pub 8: u64 uddl :,
    pub s: },
}

pub const MAX_IV_LEN: c_int = 16;
//
// struct se_crypto_request - SE crypto request structure.
// @opcode: Request opcode (enc/dec)
// @flags: flags from crypto subsystem
// @ctx_handle: Crypto context handle.
// @gph: GP Header
// @ctrl: Request Information.
// @orh: ORH address
// @comp: completion address
// @src: Input sglist
// @dst: Output sglist
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_crypto_request {
    pub opcode: u8,
    pub gfp: gfp_t,
    pub flags: u32,
    pub ctx_handle: u64,
    pub gph: gphdr,
    pub ctrl: se_req_ctrl,
    pub orh: *mut u64,
    pub comp: *mut u64,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
}

// Crypto opcodes
pub const FLEXI_CRYPTO_ENCRYPT_HMAC: c_uint = 0x33;
pub const ENCRYPT: c_int = 0;
pub const DECRYPT: c_int = 1;
// IV from context
pub const IV_FROM_CTX: c_int = 0;
// IV from Input data
pub const IV_FROM_DPTR: c_int = 1;
//
// cipher opcodes for firmware
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flexi_cipher {
    CIPHER_NULL = 0,
    CIPHER_3DES_CBC,
    CIPHER_3DES_ECB,
    CIPHER_AES_CBC,
    CIPHER_AES_ECB,
    CIPHER_AES_CFB,
    CIPHER_AES_CTR,
    CIPHER_AES_GCM,
    CIPHER_AES_XTS,
    CIPHER_AES_CCM,
    CIPHER_AES_CBC_CTS,
    CIPHER_AES_ECB_CTS,
    CIPHER_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flexi_auth {
    AUTH_NULL = 0,
    AUTH_MD5,
    AUTH_SHA1,
    AUTH_SHA2_SHA224,
    AUTH_SHA2_SHA256,
    AUTH_SHA2_SHA384,
    AUTH_SHA2_SHA512,
    AUTH_GMAC,
    AUTH_INVALID
}

//
// struct crypto_keys - Crypto keys
// @key: Encryption key or KEY1 for AES-XTS
// @iv: Encryption IV or Tweak for AES-XTS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_keys {
    pub key: [u8; AES_MAX_KEY_SIZE],
    pub key1: [u8; AES_MAX_KEY_SIZE],
    pub u: },
    pub iv: [u8; AES_BLOCK_SIZE],
}

//
// struct auth_keys - Authentication keys
// @ipad: IPAD or KEY2 for AES-XTS
// @opad: OPAD or AUTH KEY if auth_input_type = 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auth_keys {
    pub ipad: [u8; 64],
    pub key2: [u8; 64],
    pub u: },
    pub opad: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fc_ctx_flags {
    pub f: __be64,
    pub fu: u64,

    pub 4: u64 cipher_type :,
    pub 1: u64 reserved_59 :,
    pub 2: u64 aes_keylen :,
    pub 1: u64 iv_source :,
    pub 4: u64 hash_type :,
    pub 3: u64 reserved_49_51 :,
    pub 1: u64 auth_input_type:,
    pub 8: u64 mac_len :,
    pub 40: u64 reserved_0_39 :,

    pub 40: u64 reserved_0_39 :,
    pub 8: u64 mac_len :,
    pub 1: u64 auth_input_type:,
    pub 3: u64 reserved_49_51 :,
    pub 4: u64 hash_type :,
    pub 1: u64 iv_source :,
    pub 2: u64 aes_keylen :,
    pub 1: u64 reserved_59 :,
    pub 4: u64 cipher_type :,

    pub w0: },
}

//
// struct flexi_crypto_context - Crypto context
// @cipher_type: Encryption cipher type
// @aes_keylen: AES key length
// @iv_source: Encryption IV source
// @hash_type: Authentication type
// @auth_input_type: Authentication input type
// 1 - Authentication IV and KEY, microcode calculates OPAD/IPAD
// 0 - Authentication OPAD/IPAD
// @mac_len: mac length
// @crypto: Crypto keys
// @auth: Authentication keys
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexi_crypto_context {
    pub flags: fc_ctx_flags,
    pub crypto: crypto_keys,
    pub auth: auth_keys,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_ctx_hdr {
    pub pool: *mut dma_pool,
    pub dma: dma_addr_t,
    pub vaddr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_crypto_ctx {
    pub ndev: *mut nitrox_device,
    pub ctx_handle: u64,
    pub fctx: *mut flexi_crypto_context,
    pub u: },
    pub chdr: *mut crypto_ctx_hdr,
    pub callback: sereq_completion_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_kcrypt_request {
    pub creq: se_crypto_request,
    pub src: *mut u8,
    pub dst: *mut u8,
    pub iv_out: *mut u8,
}

//
// struct nitrox_aead_rctx - AEAD request context
// @nkreq: Base request context
// @cryptlen: Encryption/Decryption data length
// @assoclen: AAD length
// @srclen: Input buffer length
// @dstlen: Output buffer length
// @iv: IV data
// @ivsize: IV data length
// @flags: AEAD req flags
// @ctx_handle: Device context handle
// @src: Source sglist
// @dst: Destination sglist
// @ctrl_arg: Identifies the request type (ENCRYPT/DECRYPT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_aead_rctx {
    pub nkreq: nitrox_kcrypt_request,
    pub cryptlen: c_uint,
    pub assoclen: c_uint,
    pub srclen: c_uint,
    pub dstlen: c_uint,
    pub iv: *mut u8,
    pub ivsize: c_int,
    pub flags: u32,
    pub ctx_handle: u64,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub ctrl_arg: u8,
}

//
// struct nitrox_rfc4106_rctx - rfc4106 cipher request context
// @base: AEAD request context
// @src: Source sglist
// @dst: Destination sglist
// @assoc: AAD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_rfc4106_rctx {
    pub base: nitrox_aead_rctx,
    pub src: [scatterlist; 3],
    pub dst: [scatterlist; 3],
    pub assoc: [u8; 20],
}

//
// struct pkt_instr_hdr - Packet Instruction Header
// @g: Gather used
// When [G] is set and [GSZ] != 0, the instruction is
// indirect gather instruction.
// When [G] is set and [GSZ] = 0, the instruction is
// direct gather instruction.
// @gsz: Number of pointers in the indirect gather list
// @ihi: When set hardware duplicates the 1st 8 bytes of pkt_instr_hdr
// and adds them to the packet after the pkt_instr_hdr but before any UDD
// @ssz: Not used by the input hardware. But can become slc_store_int[SSZ]
// when [IHI] is set.
// @fsz: The number of front data bytes directly included in the
// PCIe instruction.
// @tlen: The length of the input packet in bytes, include:
// - 16B pkt_hdr
// - Inline context bytes if any,
// - UDD if any,
// - packet payload bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pkt_instr_hdr {
    pub bev: __be64,
    pub value: u64,

    pub 16: u64 raz_48_63 :,
    pub 1: u64 g :,
    pub 7: u64 gsz :,
    pub 1: u64 ihi :,
    pub 7: u64 ssz :,
    pub 2: u64 raz_30_31 :,
    pub 6: u64 fsz :,
    pub 8: u64 raz_16_23 :,
    pub 16: u64 tlen :,

    pub 16: u64 tlen :,
    pub 8: u64 raz_16_23 :,
    pub 6: u64 fsz :,
    pub 2: u64 raz_30_31 :,
    pub 7: u64 ssz :,
    pub 1: u64 ihi :,
    pub 7: u64 gsz :,
    pub 1: u64 g :,
    pub 16: u64 raz_48_63 :,

    pub s: },
}

//
// struct pkt_hdr - Packet Input Header
// @opcode: Request opcode (Major)
// @arg: Request opcode (Minor)
// @ctxc: Context control.
// @unca: When set [UNC] is the uncertainty count for an input packet.
// The hardware uses uncertainty counts to predict
// output buffer use and avoid deadlock.
// @info: Not used by input hardware. Available for use
// during SE processing.
// @destport: The expected destination port/ring/channel for the packet.
// @unc: Uncertainty count for an input packet.
// @grp: SE group that will process the input packet.
// @ctxl: Context Length in 64-bit words.
// @uddl: User-defined data (UDD) length in bytes.
// @ctxp: Context pointer. CTXP<63,2:0> must be zero in all cases.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pkt_hdr {
    pub bev: [__be64; 2],
    pub value: [u64; 2],
    pub 8: u64 opcode :,
    pub 8: u64 arg :,
    pub 2: u64 ctxc :,
    pub 1: u64 unca :,
    pub 1: u64 raz_44 :,
    pub 3: u64 info :,
    pub 9: u64 destport :,
    pub 8: u64 unc :,
    pub 5: u64 raz_19_23 :,
    pub 3: u64 grp :,
    pub 1: u64 raz_15 :,
    pub 7: u64 ctxl :,
    pub 8: u64 uddl :,

    pub 8: u64 uddl :,
    pub 7: u64 ctxl :,
    pub 1: u64 raz_15 :,
    pub 3: u64 grp :,
    pub 5: u64 raz_19_23 :,
    pub 8: u64 unc :,
    pub 9: u64 destport :,
    pub 3: u64 info :,
    pub 1: u64 raz_44 :,
    pub 1: u64 unca :,
    pub 2: u64 ctxc :,
    pub 8: u64 arg :,
    pub 8: u64 opcode :,

    pub ctxp: __be64,
    pub s: },
}

//
// struct slc_store_info - Solicited Paceket Output Store Information.
// @ssz: The number of scatterlist pointers for the solicited output port
// packet.
// @rptr: The result pointer for the solicited output port packet.
// If [SSZ]=0, [RPTR] must point directly to a buffer on the remote
// host that is large enough to hold the entire output packet.
// If [SSZ]!=0, [RPTR] must point to an array of ([SSZ]+3)/4
// sglist components at [RPTR] on the remote host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union slc_store_info {
    pub bev: [__be64; 2],
    pub value: [u64; 2],
    pub 25: u64 raz_39_63 :,
    pub 7: u64 ssz :,
    pub 32: u64 raz_0_31 :,

    pub 32: u64 raz_0_31 :,
    pub 7: u64 ssz :,
    pub 25: u64 raz_39_63 :,

    pub rptr: __be64,
    pub s: },
}

//
// struct nps_pkt_instr - NPS Packet Instruction of SE cores.
// @dptr0 : Input pointer points to buffer in remote host.
// @ih: Packet Instruction Header (8 bytes)
// @irh: Packet Input Header (16 bytes)
// @slc: Solicited Packet Output Store Information (16 bytes)
// @fdata: Front data
//
// 64-Byte Instruction Format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nps_pkt_instr {
    pub dptr0: __be64,
    pub ih: pkt_instr_hdr,
    pub irh: pkt_hdr,
    pub slc: slc_store_info,
    pub fdata: [u64; 2],
}

//
// struct aqmq_command_s - The 32 byte command for AE processing.
// @opcode: Request opcode
// @param1: Request control parameter 1
// @param2: Request control parameter 2
// @dlen: Input length
// @dptr: Input pointer points to buffer in remote host
// @rptr: Result pointer points to buffer in remote host
// @grp: AQM Group (0..7)
// @cptr: Context pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aqmq_command_s {
    pub opcode: __be16,
    pub param1: __be16,
    pub param2: __be16,
    pub dlen: __be16,
    pub dptr: __be64,
    pub rptr: __be64,
    pub word3: __be64,

    pub 3: u64 grp :,
    pub 61: u64 cptr :,

    pub 61: u64 cptr :,
    pub 3: u64 grp :,

}

//
// struct ctx_hdr - Book keeping data about the crypto context
// @pool: Pool used to allocate crypto context
// @dma: Base DMA address of the crypto context
// @ctx_dma: Actual usable crypto context for NITROX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctx_hdr {
    pub pool: *mut dma_pool,
    pub dma: dma_addr_t,
    pub ctx_dma: dma_addr_t,
}

//
// struct sglist_component - SG list component format
// @len0: The number of bytes at [PTR0] on the remote host.
// @len1: The number of bytes at [PTR1] on the remote host.
// @len2: The number of bytes at [PTR2] on the remote host.
// @len3: The number of bytes at [PTR3] on the remote host.
// @dma0: First pointer point to buffer in remote host.
// @dma1: Second pointer point to buffer in remote host.
// @dma2: Third pointer point to buffer in remote host.
// @dma3: Fourth pointer point to buffer in remote host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_sgcomp {
    pub len: [__be16; 4],
    pub dma: [__be64; 4],
}

//
// strutct nitrox_sgtable - SG list information
// @sgmap_cnt: Number of buffers mapped
// @total_bytes: Total bytes in sglist.
// @sgcomp_len: Total sglist components length.
// @sgcomp_dma: DMA address of sglist component.
// @sg: crypto request buffer.
// @sgcomp: sglist component for NITROX.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_sgtable {
    pub sgmap_cnt: u8,
    pub total_bytes: u16,
    pub sgcomp_len: u32,
    pub sgcomp_dma: dma_addr_t,
    pub sg: *mut scatterlist,
    pub sgcomp: *mut nitrox_sgcomp,
}

// Response Header Length
pub const ORH_HLEN: c_int = 8;
// Completion bytes Length
pub const COMP_HLEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resp_hdr {
    pub orh: *mut u64,
    pub completion: *mut u64,
}

extern "C" {
    pub fn void(arg: *mut *mut completion_t)(void, err: c_int) -> typedef;
}
//
// struct nitrox_softreq - Represents the NIROX Request.
// @response: response list entry
// @backlog: Backlog list entry
// @ndev: Device used to submit the request
// @cmdq: Command queue for submission
// @resp: Response headers
// @instr: 64B instruction
// @in: SG table for input
// @out SG table for output
// @tstamp: Request submitted time in jiffies
// @callback: callback after request completion/timeout
// @cb_arg: callback argument
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_softreq {
    pub response: list_head,
    pub backlog: list_head,
    pub flags: u32,
    pub gfp: gfp_t,
    pub status: core::sync::atomic::AtomicI32,
    pub ndev: *mut nitrox_device,
    pub cmdq: *mut nitrox_cmdq,
    pub instr: nps_pkt_instr,
    pub resp: resp_hdr,
    pub in: nitrox_sgtable,
    pub out: nitrox_sgtable,
    pub tstamp: c_ulong,
    pub callback: completion_t,
    pub cb_arg: *mut c_void,
}

extern "C" {
    pub fn kzalloc(_arg: size, _arg: gfp) -> return;
}
//
// create_single_sg - Point SG entry to the data
// @sg:		Destination SG list
// @buf:	Data
// @buflen:	Data length
//
// Returns next free entry in the destination SG list
//
// create_multi_sg - Create multiple sg entries with buflen data length from
// source sglist
// @to_sg:	Destination SG list
// @from_sg:	Source SG list
// @buflen:	Data length
//
// Returns next free entry in the destination SG list
//
// Input format:
// +----+----------------+
// | IV | SRC sg entries |
// +----+----------------+
//
// IV
// SRC entries
// Output format:
// +-----+----+----------------+-----------------+
// | ORH | IV | DST sg entries | COMPLETION Bytes|
// +-----+----+----------------+-----------------+
//
// ORH
// IV
// DST entries
// COMPLETION Bytes
