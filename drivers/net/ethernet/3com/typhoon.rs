//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/3com/typhoon.h
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


// typhoon.h:	chip info for the 3Com 3CR990 family of controllers
//
// All Typhoon ring positions are specificed in bytes, and point to the
// first "clean" entry in the ring -- ie the next entry we use for whatever
// purpose.
//
// The Typhoon basic ring
// ringBase:  where this ring lives (our virtual address)
// lastWrite: the next entry we'll use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_ring {
    pub ringBase: *mut u8,
    pub lastWrite: u32,
}

// The Typhoon transmit ring -- same as a basic ring, plus:
// lastRead:      where we're at in regard to cleaning up the ring
// writeRegister: register to use for writing (different for Hi & Lo rings)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transmit_ring {
    pub ringBase: *mut u8,
    pub lastWrite: u32,
    pub lastRead: u32,
    pub writeRegister: c_int,
}

// The host<->Typhoon ring index structure
// This indicates the current positions in the rings
//
// All values must be in little endian format for the 3XP
//
// rxHiCleared:   entry we've cleared to in the Hi receive ring
// rxLoCleared:   entry we've cleared to in the Lo receive ring
// rxBuffReady:   next entry we'll put a free buffer in
// respCleared:   entry we've cleared to in the response ring
//
// txLoCleared:   entry the NIC has cleared to in the Lo transmit ring
// txHiCleared:   entry the NIC has cleared to in the Hi transmit ring
// rxLoReady:     entry the NIC has filled to in the Lo receive ring
// rxBuffCleared: entry the NIC has cleared in the free buffer ring
// cmdCleared:    entry the NIC has cleared in the command ring
// respReady:     entry the NIC has filled to in the response ring
// rxHiReady:     entry the NIC has filled to in the Hi receive ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typhoon_indexes {
// The first four are written by the host, and read by the NIC
    pub rxHiCleared: volatile __le32,
    pub rxLoCleared: volatile __le32,
    pub rxBuffReady: volatile __le32,
    pub respCleared: volatile __le32,
// The remaining are written by the NIC, and read by the host
    pub txLoCleared: volatile __le32,
    pub txHiCleared: volatile __le32,
    pub rxLoReady: volatile __le32,
    pub rxBuffCleared: volatile __le32,
    pub cmdCleared: volatile __le32,
    pub respReady: volatile __le32,
    pub rxHiReady: volatile __le32,
    pub __packed: },
// The host<->Typhoon interface
// Our means of communicating where things are
//
// All values must be in little endian format for the 3XP
//
// ringIndex:   64 bit bus address of the index structure
// txLoAddr:    64 bit bus address of the Lo transmit ring
// txLoSize:    size (in bytes) of the Lo transmit ring
// txHi*:       as above for the Hi priority transmit ring
// rxLo*:       as above for the Lo priority receive ring
// rxBuff*:     as above for the free buffer ring
// cmd*:        as above for the command ring
// resp*:       as above for the response ring
// zeroAddr:    64 bit bus address of a zero word (for DMA)
// rxHi*:       as above for the Hi Priority receive ring
//
// While there is room for 64 bit addresses, current versions of the 3XP
// only do 32 bit addresses, so the *Hi for each of the above will always
// be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typhoon_interface {
    pub ringIndex: __le32,
    pub ringIndexHi: __le32,
    pub txLoAddr: __le32,
    pub txLoAddrHi: __le32,
    pub txLoSize: __le32,
    pub txHiAddr: __le32,
    pub txHiAddrHi: __le32,
    pub txHiSize: __le32,
    pub rxLoAddr: __le32,
    pub rxLoAddrHi: __le32,
    pub rxLoSize: __le32,
    pub rxBuffAddr: __le32,
    pub rxBuffAddrHi: __le32,
    pub rxBuffSize: __le32,
    pub cmdAddr: __le32,
    pub cmdAddrHi: __le32,
    pub cmdSize: __le32,
    pub respAddr: __le32,
    pub respAddrHi: __le32,
    pub respSize: __le32,
    pub zeroAddr: __le32,
    pub zeroAddrHi: __le32,
    pub rxHiAddr: __le32,
    pub rxHiAddrHi: __le32,
    pub rxHiSize: __le32,
    pub __packed: },
// The Typhoon transmit/fragment descriptor
//
// A packet is described by a packet descriptor, followed by option descriptors,
// if any, then one or more fragment descriptors.
//
// Packet descriptor:
// flags:	Descriptor type
// len:i	zero, or length of this packet
// addr*:	8 bytes of opaque data to the firmware -- for skb pointer
// processFlags: Determine offload tasks to perform on this packet.
//
// Fragment descriptor:
// flags:	Descriptor type
// len:i	length of this fragment
// addr:	low bytes of DMA address for this part of the packet
// addrHi:	hi bytes of DMA address for this part of the packet
// processFlags: must be zero
//
// TYPHOON_DESC_VALID is not mentioned in their docs, but their Linux
// driver uses it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc {
    pub flags: u8,
pub const TYPHOON_TYPE_MASK: c_uint = 0x07;
pub const TYPHOON_FRAG_DESC: c_uint = 0x00;
pub const TYPHOON_TX_DESC: c_uint = 0x01;
pub const TYPHOON_CMD_DESC: c_uint = 0x02;
pub const TYPHOON_OPT_DESC: c_uint = 0x03;
pub const TYPHOON_RX_DESC: c_uint = 0x04;
pub const TYPHOON_RESP_DESC: c_uint = 0x05;
pub const TYPHOON_OPT_TYPE_MASK: c_uint = 0xf0;
pub const TYPHOON_OPT_IPSEC: c_uint = 0x00;
pub const TYPHOON_OPT_TCP_SEG: c_uint = 0x10;
pub const TYPHOON_CMD_RESPOND: c_uint = 0x40;
pub const TYPHOON_RESP_ERROR: c_uint = 0x40;
pub const TYPHOON_RX_ERROR: c_uint = 0x40;
pub const TYPHOON_DESC_VALID: c_uint = 0x80;
    pub numDesc: u8,
    pub len: __le16,
    pub addr: __le32,
    pub addrHi: __le32,
    pub frag: },
    pub /: *mut *mut u64 tx_addr; / opaque for hardware, for TX_DESC,
}

pub const TYPHOON_TX_PF_VLAN_TAG_SHIFT: c_int = 12;
// The TCP Segmentation offload option descriptor
//
// flags:	descriptor type
// numDesc:	must be 1
// mss_flags:	bits 0-11 (little endian) are MSS, 12 is first TSO descriptor
// 13 is list TSO descriptor, set both if only one TSO
// respAddrLo:	low bytes of address of the bytesTx field of this descriptor
// bytesTx:	total number of bytes in this TSO request
// status:	0 on completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpopt_desc {
    pub flags: u8,
    pub numDesc: u8,
    pub mss_flags: __le16,

    pub respAddrLo: __le32,
    pub bytesTx: __le32,
    pub status: __le32,
    pub __packed: },
// The IPSEC Offload descriptor
//
// flags:	descriptor type
// numDesc:	must be 1
// ipsecFlags:	bit 0: 0 -- generate IV, 1 -- use supplied IV
// sa1, sa2:	Security Association IDs for this packet
// reserved:	set to 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_desc {
    pub flags: u8,
    pub numDesc: u8,
    pub ipsecFlags: __le16,

    pub sa1: __le32,
    pub sa2: __le32,
    pub reserved: __le32,
    pub __packed: },
// The Typhoon receive descriptor (Updated by NIC)
//
// flags:         Descriptor type, error indication
// numDesc:       Always zero
// frameLen:      the size of the packet received
// addr:          low 32 bytes of the virtual addr passed in for this buffer
// addrHi:        high 32 bytes of the virtual addr passed in for this buffer
// rxStatus:      Error if set in flags, otherwise result of offload processing
// filterResults: results of filtering on packet, not used
// ipsecResults:  Results of IPSEC processing
// vlanTag:       the 801.2q TCI from the packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_desc {
    pub flags: u8,
    pub numDesc: u8,
    pub frameLen: __le16,
    pub /: *mut *mut u32 addr; / opaque, comes from virtAddr,
    pub /: *mut *mut u32 addrHi; / opaque, comes from virtAddrHi,
    pub rxStatus: __le32,

    pub filterResults: __le16,

    pub ipsecResults: __le16,

    pub vlanTag: __be32,
    pub __packed: },
// The Typhoon free buffer descriptor, used to give a buffer to the NIC
//
// physAddr:    low 32 bits of the bus address of the buffer
// physAddrHi:  high 32 bits of the bus address of the buffer, always zero
// virtAddr:    low 32 bits of the skb address
// virtAddrHi:  high 32 bits of the skb address, always zero
//
// the virt* address is basically two 32 bit cookies, just passed back
// from the NIC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_free {
    pub physAddr: __le32,
    pub physAddrHi: __le32,
    pub virtAddr: u32,
    pub virtAddrHi: u32,
    pub __packed: },
// The Typhoon command descriptor, used for commands and responses
//
// flags:   descriptor type
// numDesc: number of descriptors following in this command/response,
// ie, zero for a one descriptor command
// cmd:     the command
// seqNo:   sequence number (unused)
// parm1:   use varies by command
// parm2:   use varies by command
// parm3:   use varies by command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_desc {
    pub flags: u8,
    pub numDesc: u8,
    pub cmd: __le16,

    pub seqNo: u16,
    pub parm1: __le16,
    pub parm2: __le32,
    pub parm3: __le32,
    pub __packed: },
// The Typhoon response descriptor, see command descriptor for details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resp_desc {
    pub flags: u8,
    pub numDesc: u8,
    pub cmd: __le16,
    pub seqNo: __le16,
    pub parm1: __le16,
    pub parm2: __le32,
    pub parm3: __le32,
    pub __packed: },

    pub \: *mut *mut do { struct cmd_desc _ptr = (x);,
    pub \: memset(_ptr, 0, sizeof(struct cmd_desc));,
    pub \: _ptr->flags = TYPHOON_CMD_DESC | TYPHOON_DESC_VALID;,
    pub \: _ptr->cmd = command;,
// We set seqNo to 1 if we're expecting a response from this command

    pub \: *mut *mut do { struct cmd_desc _ptr = (x);,
    pub \: memset(_ptr, 0, sizeof(struct cmd_desc));,
    pub \: _ptr->flags = TYPHOON_CMD_RESPOND | TYPHOON_CMD_DESC;,
    pub \: _ptr->flags |= TYPHOON_DESC_VALID;,
    pub \: _ptr->cmd = command;,
    pub \: _ptr->seqNo = 1;,
// TYPHOON_CMD_SET_RX_FILTER filter bits (cmd.parm1)
//

// TYPHOON_CMD_READ_STATS response format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_resp {
    pub flags: u8,
    pub numDesc: u8,
    pub cmd: __le16,
    pub seqNo: __le16,
    pub unused: __le16,
    pub txPackets: __le32,
    pub txBytes: __le64,
    pub txDeferred: __le32,
    pub txLateCollisions: __le32,
    pub txCollisions: __le32,
    pub txCarrierLost: __le32,
    pub txMultipleCollisions: __le32,
    pub txExcessiveCollisions: __le32,
    pub txFifoUnderruns: __le32,
    pub txMulticastTxOverflows: __le32,
    pub txFiltered: __le32,
    pub rxPacketsGood: __le32,
    pub rxBytesGood: __le64,
    pub rxFifoOverruns: __le32,
    pub BadSSD: __le32,
    pub rxCrcErrors: __le32,
    pub rxOversized: __le32,
    pub rxBroadcast: __le32,
    pub rxMulticast: __le32,
    pub rxOverflow: __le32,
    pub rxFiltered: __le32,
    pub linkStatus: __le32,

    pub unused2: __le32,
    pub unused3: __le32,
    pub __packed: },
// TYPHOON_CMD_XCVR_SELECT xcvr values (resp.parm1)
//

// TYPHOON_CMD_READ_MEDIA_STATUS (resp.parm1)
//

// TYPHOON_CMD_SET_MULTICAST_HASH enable values (cmd.parm1)
//

// TYPHOON_CMD_CREATE_SA descriptor and settings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_descriptor {
    pub flags: u8,
    pub numDesc: u8,
    pub cmd: u16,
    pub seqNo: u16,
    pub mode: u16,

    pub hashFlags: u8,
pub const TYPHOON_SA_HASH_ENABLE: c_uint = 0x01;
pub const TYPHOON_SA_HASH_SHA1: c_uint = 0x02;
pub const TYPHOON_SA_HASH_MD5: c_uint = 0x04;
    pub direction: u8,
pub const TYPHOON_SA_DIR_RX: c_uint = 0x00;
pub const TYPHOON_SA_DIR_TX: c_uint = 0x01;
    pub encryptionFlags: u8,
pub const TYPHOON_SA_ENCRYPT_ENABLE: c_uint = 0x01;
pub const TYPHOON_SA_ENCRYPT_DES: c_uint = 0x02;
pub const TYPHOON_SA_ENCRYPT_3DES: c_uint = 0x00;
pub const TYPHOON_SA_ENCRYPT_3DES_2KEY: c_uint = 0x00;
pub const TYPHOON_SA_ENCRYPT_3DES_3KEY: c_uint = 0x04;
pub const TYPHOON_SA_ENCRYPT_CBC: c_uint = 0x08;
pub const TYPHOON_SA_ENCRYPT_ECB: c_uint = 0x00;
    pub specifyIndex: u8,
pub const TYPHOON_SA_SPECIFY_INDEX: c_uint = 0x01;
pub const TYPHOON_SA_GENERATE_INDEX: c_uint = 0x00;
    pub SPI: u32,
    pub destAddr: u32,
    pub destMask: u32,
    pub integKey: [u8; 20],
    pub confKey: [u8; 24],
    pub index: u32,
    pub unused: u32,
    pub unused2: u32,
    pub __packed: },
// TYPHOON_CMD_SET_OFFLOAD_TASKS bits (cmd.parm2 (Tx) & cmd.parm3 (Rx))
// This is all for IPv4.
//

// TYPHOON_CMD_ENABLE_WAKE_EVENTS bits (cmd.parm1)
//

// These are used to load the firmware image on the NIC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typhoon_file_header {
    pub tag: [u8; 8],
    pub version: __le32,
    pub numSections: __le32,
    pub startAddr: __le32,
    pub hmacDigest: [__le32; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typhoon_section_header {
    pub len: __le32,
    pub checksum: u16,
    pub reserved: u16,
    pub startAddr: __le32,
    pub __packed: },
// The Typhoon Register offsets
//
pub const TYPHOON_REG_SOFT_RESET: c_uint = 0x00;
pub const TYPHOON_REG_INTR_STATUS: c_uint = 0x04;
pub const TYPHOON_REG_INTR_ENABLE: c_uint = 0x08;
pub const TYPHOON_REG_INTR_MASK: c_uint = 0x0c;
pub const TYPHOON_REG_SELF_INTERRUPT: c_uint = 0x10;
pub const TYPHOON_REG_HOST2ARM7: c_uint = 0x14;
pub const TYPHOON_REG_HOST2ARM6: c_uint = 0x18;
pub const TYPHOON_REG_HOST2ARM5: c_uint = 0x1c;
pub const TYPHOON_REG_HOST2ARM4: c_uint = 0x20;
pub const TYPHOON_REG_HOST2ARM3: c_uint = 0x24;
pub const TYPHOON_REG_HOST2ARM2: c_uint = 0x28;
pub const TYPHOON_REG_HOST2ARM1: c_uint = 0x2c;
pub const TYPHOON_REG_HOST2ARM0: c_uint = 0x30;
pub const TYPHOON_REG_ARM2HOST3: c_uint = 0x34;
pub const TYPHOON_REG_ARM2HOST2: c_uint = 0x38;
pub const TYPHOON_REG_ARM2HOST1: c_uint = 0x3c;
pub const TYPHOON_REG_ARM2HOST0: c_uint = 0x40;

// 3XP Reset values (TYPHOON_REG_SOFT_RESET)
//
pub const TYPHOON_RESET_ALL: c_uint = 0x7f;
pub const TYPHOON_RESET_NONE: c_uint = 0x00;
// 3XP irq bits (TYPHOON_REG_INTR{STATUS,ENABLE,MASK})
//
// Some of these came from OpenBSD, as the 3Com docs have it wrong
// (INTR_SELF) or don't list it at all (INTR_*_ABORT)
//
// Enabling irqs on the Heartbeat reg (ArmToHost3) gets you an irq
// about every 8ms, so don't do it.
//
pub const TYPHOON_INTR_HOST_INT: c_uint = 0x00000001;
pub const TYPHOON_INTR_ARM2HOST0: c_uint = 0x00000002;
pub const TYPHOON_INTR_ARM2HOST1: c_uint = 0x00000004;
pub const TYPHOON_INTR_ARM2HOST2: c_uint = 0x00000008;
pub const TYPHOON_INTR_ARM2HOST3: c_uint = 0x00000010;
pub const TYPHOON_INTR_DMA0: c_uint = 0x00000020;
pub const TYPHOON_INTR_DMA1: c_uint = 0x00000040;
pub const TYPHOON_INTR_DMA2: c_uint = 0x00000080;
pub const TYPHOON_INTR_DMA3: c_uint = 0x00000100;
pub const TYPHOON_INTR_MASTER_ABORT: c_uint = 0x00000200;
pub const TYPHOON_INTR_TARGET_ABORT: c_uint = 0x00000400;
pub const TYPHOON_INTR_SELF: c_uint = 0x00000800;
pub const TYPHOON_INTR_RESERVED: c_uint = 0xfffff000;

pub const TYPHOON_INTR_ENABLE_ALL: c_uint = 0xffffffef;
pub const TYPHOON_INTR_ALL: c_uint = 0xffffffff;
pub const TYPHOON_INTR_NONE: c_uint = 0x00000000;
// The commands for the 3XP chip (TYPHOON_REG_COMMAND)
//
pub const TYPHOON_BOOTCMD_BOOT: c_uint = 0x00;
pub const TYPHOON_BOOTCMD_WAKEUP: c_uint = 0xfa;
pub const TYPHOON_BOOTCMD_DNLD_COMPLETE: c_uint = 0xfb;
pub const TYPHOON_BOOTCMD_SEG_AVAILABLE: c_uint = 0xfc;
pub const TYPHOON_BOOTCMD_RUNTIME_IMAGE: c_uint = 0xfd;
pub const TYPHOON_BOOTCMD_REG_BOOT_RECORD: c_uint = 0xff;
// 3XP Status values (TYPHOON_REG_STATUS)
//
pub const TYPHOON_STATUS_WAITING_FOR_BOOT: c_uint = 0x07;
pub const TYPHOON_STATUS_SECOND_INIT: c_uint = 0x08;
pub const TYPHOON_STATUS_RUNNING: c_uint = 0x09;
pub const TYPHOON_STATUS_WAITING_FOR_HOST: c_uint = 0x0d;
pub const TYPHOON_STATUS_WAITING_FOR_SEGMENT: c_uint = 0x10;
pub const TYPHOON_STATUS_SLEEPING: c_uint = 0x11;
pub const TYPHOON_STATUS_HALTED: c_uint = 0x14;
