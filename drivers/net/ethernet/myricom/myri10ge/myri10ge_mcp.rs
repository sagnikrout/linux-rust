//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/myricom/myri10ge/myri10ge_mcp.h
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
pub const MXGEFW_VERSION_MAJOR: c_int = 1;
pub const MXGEFW_VERSION_MINOR: c_int = 4;
// 8 Bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_dma_addr {
    pub high: __be32,
    pub low: __be32,
}

// 4 Bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_slot {
    pub checksum: __sum16,
    pub length: __be16,
}

// 64 Bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_cmd {
    pub cmd: __be32,
    pub /: *mut *mut __be32 data0; / will be low portion if data > 32 bits,
// 8
    pub /: *mut *mut __be32 data1; / will be high portion if data > 32 bits,
    pub /: *mut *mut __be32 data2; / currently unused..,
// 16
    pub response_addr: mcp_dma_addr,
// 24
    pub pad: [u8; 40],
}

// 8 Bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_cmd_response {
    pub data: __be32,
    pub result: __be32,
}

//
// flags used in mcp_kreq_ether_send_t:
//
// The SMALL flag is only needed in the first segment. It is raised
// for packets that are total less or equal 512 bytes.
//
// The CKSUM flag must be set in all segments.
//
// The PADDED flags is set if the packet needs to be padded, and it
// must be set for all segments.
//
// The  MXGEFW_FLAGS_ALIGN_ODD must be set if the cumulative
// length of all previous segments was odd.
//
pub const MXGEFW_FLAGS_SMALL: c_uint = 0x1;
pub const MXGEFW_FLAGS_TSO_HDR: c_uint = 0x1;
pub const MXGEFW_FLAGS_FIRST: c_uint = 0x2;
pub const MXGEFW_FLAGS_ALIGN_ODD: c_uint = 0x4;
pub const MXGEFW_FLAGS_CKSUM: c_uint = 0x8;
pub const MXGEFW_FLAGS_TSO_LAST: c_uint = 0x8;
pub const MXGEFW_FLAGS_NO_TSO: c_uint = 0x10;
pub const MXGEFW_FLAGS_TSO_CHOP: c_uint = 0x10;
pub const MXGEFW_FLAGS_TSO_PLD: c_uint = 0x20;
pub const MXGEFW_SEND_SMALL_SIZE: c_int = 1520;
pub const MXGEFW_MAX_MTU: c_int = 9400;
#[repr(C)]
#[derive(Copy, Clone)]
pub union mcp_pso_or_cumlen {
    pub pseudo_hdr_offset: u16,
    pub cum_len: u16,
}

pub const MXGEFW_MAX_SEND_DESC: c_int = 12;
pub const MXGEFW_PAD: c_int = 2;
// 16 Bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_kreq_ether_send {
    pub addr_high: __be32,
    pub addr_low: __be32,
    pub pseudo_hdr_offset: __be16,
    pub length: __be16,
    pub pad: u8,
    pub rdma_count: u8,
    pub /: *mut *mut u8 cksum_offset; / where to start computing cksum,
    pub /: *mut *mut u8 flags; / as defined above,
}

// 8 Bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_kreq_ether_recv {
    pub addr_high: __be32,
    pub addr_low: __be32,
}

// Commands
pub const MXGEFW_BOOT_HANDOFF: c_uint = 0xfc0000;
pub const MXGEFW_BOOT_DUMMY_RDMA: c_uint = 0xfc01c0;
pub const MXGEFW_ETH_CMD: c_uint = 0xf80000;
pub const MXGEFW_ETH_SEND_4: c_uint = 0x200000;
pub const MXGEFW_ETH_SEND_1: c_uint = 0x240000;
pub const MXGEFW_ETH_SEND_2: c_uint = 0x280000;
pub const MXGEFW_ETH_SEND_3: c_uint = 0x2c0000;
pub const MXGEFW_ETH_RECV_SMALL: c_uint = 0x300000;
pub const MXGEFW_ETH_RECV_BIG: c_uint = 0x340000;
pub const MXGEFW_ETH_SEND_GO: c_uint = 0x380000;
pub const MXGEFW_ETH_SEND_STOP: c_uint = 0x3C0000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum myri10ge_mcp_cmd_type {
    MXGEFW_CMD_NONE = 0,
// Reset the mcp, it is left in a safe state, waiting
// for the driver to set all its parameters
    MXGEFW_CMD_RESET = 1,

// get the version number of the current firmware..
// (may be available in the eeprom strings..?
    MXGEFW_GET_MCP_VERSION = 2,

// Parameters which must be set by the driver before it can
// issue MXGEFW_CMD_ETHERNET_UP. They persist until the next
// MXGEFW_CMD_RESET is issued

    MXGEFW_CMD_SET_INTRQ_DMA = 3,
// data0 = LSW of the host address
// data1 = MSW of the host address
// data2 = slice number if multiple slices are used
//

    MXGEFW_CMD_SET_BIG_BUFFER_SIZE = 4,	/* in bytes, power of 2 */
    MXGEFW_CMD_SET_SMALL_BUFFER_SIZE = 5,	/* in bytes */

// Parameters which refer to lanai SRAM addresses where the
// driver must issue PIO writes for various things

    MXGEFW_CMD_GET_SEND_OFFSET = 6,
    MXGEFW_CMD_GET_SMALL_RX_OFFSET = 7,
    MXGEFW_CMD_GET_BIG_RX_OFFSET = 8,
// data0 = slice number if multiple slices are used

    MXGEFW_CMD_GET_IRQ_ACK_OFFSET = 9,
    MXGEFW_CMD_GET_IRQ_DEASSERT_OFFSET = 10,

// Parameters which refer to rings stored on the MCP,
// and whose size is controlled by the mcp

    MXGEFW_CMD_GET_SEND_RING_SIZE = 11,	/* in bytes */
    MXGEFW_CMD_GET_RX_RING_SIZE = 12,	/* in bytes */

// Parameters which refer to rings stored in the host,
// and whose size is controlled by the host.  Note that
// all must be physically contiguous and must contain
// a power of 2 number of entries.

    MXGEFW_CMD_SET_INTRQ_SIZE = 13,	/* in bytes */

// command to bring ethernet interface up.  Above parameters
// (plus mtu & mac address) must have been exchanged prior
// to issuing this command
    MXGEFW_CMD_ETHERNET_UP = 14,

// command to bring ethernet interface down.  No further sends
// or receives may be processed until an MXGEFW_CMD_ETHERNET_UP
// is issued, and all interrupt queues must be flushed prior
// to ack'ing this command

    MXGEFW_CMD_ETHERNET_DOWN = 15,

// commands the driver may issue live, without resetting
// the nic.  Note that increasing the mtu "live" should
// only be done if the driver has already supplied buffers
// sufficiently large to handle the new mtu.  Decreasing
// the mtu live is safe

    MXGEFW_CMD_SET_MTU = 16,
    MXGEFW_CMD_GET_INTR_COAL_DELAY_OFFSET = 17,	/* in microseconds */
    MXGEFW_CMD_SET_STATS_INTERVAL = 18,	/* in microseconds */
    MXGEFW_CMD_SET_STATS_DMA_OBSOLETE = 19,	/* replaced by SET_STATS_DMA_V2 */

    MXGEFW_ENABLE_PROMISC = 20,
    MXGEFW_DISABLE_PROMISC = 21,
    MXGEFW_SET_MAC_ADDRESS = 22,

    MXGEFW_ENABLE_FLOW_CONTROL = 23,
    MXGEFW_DISABLE_FLOW_CONTROL = 24,

// do a DMA test
// data0,data1 = DMA address
// data2       = RDMA length (MSH), WDMA length (LSH)
// command return data = repetitions (MSH), 0.5-ms ticks (LSH)
//
    MXGEFW_DMA_TEST = 25,

    MXGEFW_ENABLE_ALLMULTI = 26,
    MXGEFW_DISABLE_ALLMULTI = 27,

// returns MXGEFW_CMD_ERROR_MULTICAST
// if there is no room in the cache
// data0,MSH(data1) = multicast group address
    MXGEFW_JOIN_MULTICAST_GROUP = 28,
// returns MXGEFW_CMD_ERROR_MULTICAST
// if the address is not in the cache,
// or is equal to FF-FF-FF-FF-FF-FF
// data0,MSH(data1) = multicast group address
    MXGEFW_LEAVE_MULTICAST_GROUP = 29,
    MXGEFW_LEAVE_ALL_MULTICAST_GROUPS = 30,

    MXGEFW_CMD_SET_STATS_DMA_V2 = 31,
// data0, data1 = bus addr,
// data2 = sizeof(struct mcp_irq_data) from driver point of view, allows
// adding new stuff to mcp_irq_data without changing the ABI
//
// If multiple slices are used, data2 contains both the size of the
// structure (in the lower 16 bits) and the slice number
// (in the upper 16 bits).
//

    MXGEFW_CMD_UNALIGNED_TEST = 32,
// same than DMA_TEST (same args) but abort with UNALIGNED on unaligned
// chipset

    MXGEFW_CMD_UNALIGNED_STATUS = 33,
// return data = boolean, true if the chipset is known to be unaligned

    MXGEFW_CMD_ALWAYS_USE_N_BIG_BUFFERS = 34,
// data0 = number of big buffers to use.  It must be 0 or a power of 2.
// 0 indicates that the NIC consumes as many buffers as they are required
// for packet. This is the default behavior.
// A power of 2 number indicates that the NIC always uses the specified
// number of buffers for each big receive packet.
// It is up to the driver to ensure that this value is big enough for
// the NIC to be able to receive maximum-sized packets.
//

    MXGEFW_CMD_GET_MAX_RSS_QUEUES = 35,
    MXGEFW_CMD_ENABLE_RSS_QUEUES = 36,
// data0 = number of slices n (0, 1, ..., n-1) to enable
// data1 = interrupt mode | use of multiple transmit queues.
// 0=share one INTx/MSI.
// 1=use one MSI-X per queue.
// If all queues share one interrupt, the driver must have set
// RSS_SHARED_INTERRUPT_DMA before enabling queues.
// 2=enable both receive and send queues.
// Without this bit set, only one send queue (slice 0's send queue)
// is enabled.  The receive queues are always enabled.
//
pub const MXGEFW_SLICE_INTR_MODE_SHARED: c_uint = 0x0;
pub const MXGEFW_SLICE_INTR_MODE_ONE_PER_SLICE: c_uint = 0x1;
pub const MXGEFW_SLICE_ENABLE_MULTIPLE_TX_QUEUES: c_uint = 0x2;

    MXGEFW_CMD_GET_RSS_SHARED_INTERRUPT_MASK_OFFSET = 37,
    MXGEFW_CMD_SET_RSS_SHARED_INTERRUPT_DMA = 38,
// data0, data1 = bus address lsw, msw
    MXGEFW_CMD_GET_RSS_TABLE_OFFSET = 39,
// get the offset of the indirection table
    MXGEFW_CMD_SET_RSS_TABLE_SIZE = 40,
// set the size of the indirection table
    MXGEFW_CMD_GET_RSS_KEY_OFFSET = 41,
// get the offset of the secret key
    MXGEFW_CMD_RSS_KEY_UPDATED = 42,
// tell nic that the secret key's been updated
    MXGEFW_CMD_SET_RSS_ENABLE = 43,
// data0 = enable/disable rss
// 0: disable rss.  nic does not distribute receive packets.
// 1: enable rss.  nic distributes receive packets among queues.
// data1 = hash type
// 1: IPV4            (required by RSS)
// 2: TCP_IPV4        (required by RSS)
// 3: IPV4 | TCP_IPV4 (required by RSS)
// 4: source port
// 5: source port + destination port
//
pub const MXGEFW_RSS_HASH_TYPE_IPV4: c_uint = 0x1;
pub const MXGEFW_RSS_HASH_TYPE_TCP_IPV4: c_uint = 0x2;
pub const MXGEFW_RSS_HASH_TYPE_SRC_PORT: c_uint = 0x4;
pub const MXGEFW_RSS_HASH_TYPE_SRC_DST_PORT: c_uint = 0x5;
pub const MXGEFW_RSS_HASH_TYPE_MAX: c_uint = 0x5;

    MXGEFW_CMD_GET_MAX_TSO6_HDR_SIZE = 44,
// Return data = the max. size of the entire headers of a IPv6 TSO packet.
// If the header size of a IPv6 TSO packet is larger than the specified
// value, then the driver must not use TSO.
// This size restriction only applies to IPv6 TSO.
// For IPv4 TSO, the maximum size of the headers is fixed, and the NIC
// always has enough header buffer to store maximum-sized headers.
//

    MXGEFW_CMD_SET_TSO_MODE = 45,
// data0 = TSO mode.
// 0: Linux/FreeBSD style (NIC default)
// 1: NDIS/NetBSD style
//
pub const MXGEFW_TSO_MODE_LINUX: c_int = 0;
pub const MXGEFW_TSO_MODE_NDIS: c_int = 1;

    MXGEFW_CMD_MDIO_READ = 46,
// data0 = dev_addr (PMA/PMD or PCS ...), data1 = register/addr
    MXGEFW_CMD_MDIO_WRITE = 47,
// data0 = dev_addr,  data1 = register/addr, data2 = value

    MXGEFW_CMD_I2C_READ = 48,
// Starts to get a fresh copy of one byte or of the module i2c table, the
// obtained data is cached inside the xaui-xfi chip :
// data0 :  0 => get one byte, 1=> get 256 bytes
// data1 :  If data0 == 0: location to refresh
// bit 7:0  register location
// bit 8:15 is the i2c slave addr (0 is interpreted as 0xA1)
// bit 23:16 is the i2c bus number (for multi-port NICs)
// If data0 == 1: unused
// The operation might take ~1ms for a single byte or ~65ms when refreshing all 256 bytes
// During the i2c operation,  MXGEFW_CMD_I2C_READ or MXGEFW_CMD_I2C_BYTE attempts
// will return MXGEFW_CMD_ERROR_BUSY
//
    MXGEFW_CMD_I2C_BYTE = 49,
// Return the last obtained copy of a given byte in the xfp i2c table
// (copy cached during the last relevant MXGEFW_CMD_I2C_READ)
// data0 : index of the desired table entry
// Return data = the byte stored at the requested index in the table
//

    MXGEFW_CMD_GET_VPUMP_OFFSET = 50,
// Return data = NIC memory offset of mcp_vpump_public_global
    MXGEFW_CMD_RESET_VPUMP = 51,
// Resets the VPUMP state

    MXGEFW_CMD_SET_RSS_MCP_SLOT_TYPE = 52,
// data0 = mcp_slot type to use.
// 0 = the default 4B mcp_slot
// 1 = 8B mcp_slot_8
//
pub const MXGEFW_RSS_MCP_SLOT_TYPE_MIN: c_int = 0;
pub const MXGEFW_RSS_MCP_SLOT_TYPE_WITH_HASH: c_int = 1;

    MXGEFW_CMD_SET_THROTTLE_FACTOR = 53,
// set the throttle factor for ethp_z8e
// data0 = throttle_factor
// throttle_factor = 256 * pcie-raw-speed / tx_speed
// tx_speed = 256 * pcie-raw-speed / throttle_factor
//
// For PCI-E x8: pcie-raw-speed == 16Gb/s
// For PCI-E x4: pcie-raw-speed == 8Gb/s
//
// ex1: throttle_factor == 0x1a0 (416), tx_speed == 1.23GB/s == 9.846 Gb/s
// ex2: throttle_factor == 0x200 (512), tx_speed == 1.0GB/s == 8 Gb/s
//
// with tx_boundary == 2048, max-throttle-factor == 8191 => min-speed == 500Mb/s
// with tx_boundary == 4096, max-throttle-factor == 4095 => min-speed == 1Gb/s
//

    MXGEFW_CMD_VPUMP_UP = 54,
// Allocates VPump Connection, Send Request and Zero copy buffer address tables
    MXGEFW_CMD_GET_VPUMP_CLK = 55,
// Get the lanai clock

    MXGEFW_CMD_GET_DCA_OFFSET = 56,
// offset of dca control for WDMAs

// VMware NetQueue commands
    MXGEFW_CMD_NETQ_GET_FILTERS_PER_QUEUE = 57,
    MXGEFW_CMD_NETQ_ADD_FILTER = 58,
// data0 = filter_id << 16 | queue << 8 | type
// data1 = MS4 of MAC Addr
// data2 = LS2_MAC << 16 | VLAN_tag
    MXGEFW_CMD_NETQ_DEL_FILTER = 59,
// data0 = filter_id
    MXGEFW_CMD_NETQ_QUERY1 = 60,
    MXGEFW_CMD_NETQ_QUERY2 = 61,
    MXGEFW_CMD_NETQ_QUERY3 = 62,
    MXGEFW_CMD_NETQ_QUERY4 = 63,

    MXGEFW_CMD_RELAX_RXBUFFER_ALIGNMENT = 64,
// When set, small receive buffers can cross page boundaries.
// Both small and big receive buffers may start at any address.
// This option has performance implications, so use with caution.
//
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum myri10ge_mcp_cmd_status {
    MXGEFW_CMD_OK = 0,
    MXGEFW_CMD_UNKNOWN = 1,
    MXGEFW_CMD_ERROR_RANGE = 2,
    MXGEFW_CMD_ERROR_BUSY = 3,
    MXGEFW_CMD_ERROR_EMPTY = 4,
    MXGEFW_CMD_ERROR_CLOSED = 5,
    MXGEFW_CMD_ERROR_HASH_ERROR = 6,
    MXGEFW_CMD_ERROR_BAD_PORT = 7,
    MXGEFW_CMD_ERROR_RESOURCES = 8,
    MXGEFW_CMD_ERROR_MULTICAST = 9,
    MXGEFW_CMD_ERROR_UNALIGNED = 10,
    MXGEFW_CMD_ERROR_NO_MDIO = 11,
    MXGEFW_CMD_ERROR_I2C_FAILURE = 12,
    MXGEFW_CMD_ERROR_I2C_ABSENT = 13,
    MXGEFW_CMD_ERROR_BAD_PCIE_LINK = 14
}

pub const MXGEFW_OLD_IRQ_DATA_LEN: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_irq_data {
// add new counters at the beginning
    pub future_use: [__be32; 1],
    pub dropped_pause: __be32,
    pub dropped_unicast_filtered: __be32,
    pub dropped_bad_crc32: __be32,
    pub dropped_bad_phy: __be32,
    pub dropped_multicast_filtered: __be32,
// 40 Bytes
    pub send_done_count: __be32,
pub const MXGEFW_LINK_DOWN: c_int = 0;
pub const MXGEFW_LINK_UP: c_int = 1;
pub const MXGEFW_LINK_MYRINET: c_int = 2;
pub const MXGEFW_LINK_UNKNOWN: c_int = 3;
    pub link_up: __be32,
    pub dropped_link_overflow: __be32,
    pub dropped_link_error_or_filtered: __be32,
    pub dropped_runt: __be32,
    pub dropped_overrun: __be32,
    pub dropped_no_small_buffer: __be32,
    pub dropped_no_big_buffer: __be32,
    pub rdma_tags_available: __be32,
    pub tx_stopped: u8,
    pub link_down: u8,
    pub stats_updated: u8,
    pub valid: u8,
}

// definitions for NETQ filter type
pub const MXGEFW_NETQ_FILTERTYPE_NONE: c_int = 0;
pub const MXGEFW_NETQ_FILTERTYPE_MACADDR: c_int = 1;
pub const MXGEFW_NETQ_FILTERTYPE_VLAN: c_int = 2;
pub const MXGEFW_NETQ_FILTERTYPE_VLANMACADDR: c_int = 3;
