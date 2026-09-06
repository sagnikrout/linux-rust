//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_net.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2018 Netronome Systems, Inc.
//
// nfp_net.h
// Declarations for Netronome network device driver.
// Authors: Jakub Kicinski <jakub.kicinski@netronome.com>
// Jason McMullan <jason.mcmullan@netronome.com>
// Rolf Neugebauer <rolf.neugebauer@netronome.com>
//

// Max time to wait for NFP to respond on updates (in seconds)
pub const NFP_NET_POLL_TIMEOUT: c_int = 5;
// Interval for reading offloaded filter stats

// Bar allocation
pub const NFP_NET_CTRL_BAR: c_int = 0;
pub const NFP_NET_Q0_BAR: c_int = 2;

// Default size for MTU and freelist buffer sizes

// Maximum number of bytes prepended to a packet
pub const NFP_NET_MAX_PREPEND: c_int = 64;
// Interrupt definitions
pub const NFP_NET_NON_Q_VECTORS: c_int = 2;
pub const NFP_NET_IRQ_LSC_IDX: c_int = 0;
pub const NFP_NET_IRQ_EXN_IDX: c_int = 1;

// Queue/Ring definitions

// MC definitions

// Offload definitions

// Forward declarations
// Convenience macro for wrapping descriptor index on ring size

// Convenience macro for writing dma address into RX/TX descriptors

//
// struct nfp_net_tx_ring - TX ring structure
// @r_vec:      Back pointer to ring vector structure
// @idx:        Ring index from Linux's perspective
// @data_pending: number of bytes added to current block (NFDK only)
// @qcp_q:      Pointer to base of the QCP TX queue
// @txrwb:	TX pointer write back area
// @cnt:        Size of the queue in number of descriptors
// @wr_p:       TX ring write pointer (free running)
// @rd_p:       TX ring read pointer (free running)
// @qcp_rd_p:   Local copy of QCP TX queue read pointer
// @wr_ptr_add:	Accumulated number of buffers to add to QCP write pointer
// (used for .xmit_more delayed kick)
// @txbufs:	Array of transmitted TX buffers, to free on transmit (NFD3)
// @ktxbufs:	Array of transmitted TX buffers, to free on transmit (NFDK)
// @txds:	Virtual address of TX ring in host memory (NFD3)
// @ktxds:	Virtual address of TX ring in host memory (NFDK)
//
// @qcidx:      Queue Controller Peripheral (QCP) queue index for the TX queue
// @dma:        DMA address of the TX ring
// @size:       Size, in bytes, of the TX ring (needed to free)
// @is_xdp:	Is this a XDP TX ring?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_tx_ring {
    pub r_vec: *mut nfp_net_r_vector,
    pub idx: u16,
    pub data_pending: u16,
    pub qcp_q: *mut u8 __iomem,
    pub txrwb: *mut u64,
    pub cnt: u32,
    pub wr_p: u32,
    pub rd_p: u32,
    pub qcp_rd_p: u32,
    pub wr_ptr_add: u32,
    pub txbufs: *mut nfp_nfd3_tx_buf,
    pub ktxbufs: *mut nfp_nfdk_tx_buf,
}

// Cold data follows
// RX and freelist descriptor format

// Flags in the RX descriptor

pub const PCIE_DESC_RX_CSUM_OK_SHIFT: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_rx_desc {
    pub /: *mut *mut __le16 dma_addr_hi; / High bits of the buf address,
    pub /: *mut *mut u8 reserved; / Must be zero,
    pub /: *mut *mut u8 meta_len_dd; / Must be zero,
    pub /: *mut *mut __le32 dma_addr_lo; / Low bits of the buffer address,
    pub fld: } __packed,
    pub /: *mut *mut __le16 data_len; / Length of the frame + meta data,
    pub reserved: u8,
    pub +: *mut *mut u8 meta_len_dd; / Length of meta data prepended,
// descriptor done flag.
//
    pub /: *mut *mut *mut __le16 flags; / RX flags. See @PCIE_DESC_RX_,
    pub /: *mut *mut __le16 vlan; / VLAN if stripped,
    pub rxd: } __packed,
    pub vals: [__le32; 2],
}

pub const NFP_NET_VLAN_CTAG: c_int = 0;
pub const NFP_NET_VLAN_STAG: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_meta_parsed {
    pub hash_type: u8,
    pub csum_type: u8,
    pub hash: u32,
    pub mark: u32,
    pub portid: u32,
    pub csum: __wsum,
    pub stripped: bool,
    pub tpid: u8,
    pub tci: u16,
    pub vlan: },

    pub ipsec_saidx: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_rx_hash {
    pub hash_type: __be32,
    pub hash: __be32,
}

//
// struct nfp_net_rx_buf - software RX buffer descriptor
// @frag:	page fragment buffer
// @dma_addr:	DMA mapping address of the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_rx_buf {
    pub frag: *mut c_void,
    pub dma_addr: dma_addr_t,
}

//
// struct nfp_net_xsk_rx_buf - software RX XSK buffer descriptor
// @dma_addr:	DMA mapping address of the buffer
// @xdp:	XSK buffer pool handle (for AF_XDP)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_xsk_rx_buf {
    pub dma_addr: dma_addr_t,
    pub xdp: *mut xdp_buff,
}

//
// struct nfp_net_rx_ring - RX ring structure
// @r_vec:      Back pointer to ring vector structure
// @cnt:        Size of the queue in number of descriptors
// @wr_p:       FL/RX ring write pointer (free running)
// @rd_p:       FL/RX ring read pointer (free running)
// @idx:        Ring index from Linux's perspective
// @fl_qcidx:   Queue Controller Peripheral (QCP) queue index for the freelist
// @qcp_fl:     Pointer to base of the QCP freelist queue
// @rxbufs:     Array of transmitted FL/RX buffers
// @xsk_rxbufs: Array of transmitted FL/RX buffers (for AF_XDP)
// @rxds:       Virtual address of FL/RX ring in host memory
// @xdp_rxq:    RX-ring info avail for XDP
// @dma:        DMA address of the FL/RX ring
// @size:       Size, in bytes, of the FL/RX ring (needed to free)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_rx_ring {
    pub r_vec: *mut nfp_net_r_vector,
    pub cnt: u32,
    pub wr_p: u32,
    pub rd_p: u32,
    pub idx: u32,
    pub fl_qcidx: c_int,
    pub qcp_fl: *mut u8 __iomem,
    pub rxbufs: *mut nfp_net_rx_buf,
    pub xsk_rxbufs: *mut nfp_net_xsk_rx_buf,
    pub rxds: *mut nfp_net_rx_desc,
    pub xdp_rxq: xdp_rxq_info,
    pub dma: dma_addr_t,
    pub size: usize,
    pub ____cacheline_aligned: },
//
// struct nfp_net_r_vector - Per ring interrupt vector configuration
// @nfp_net:        Backpointer to nfp_net structure
// @napi:           NAPI structure for this ring vec
// @tasklet:        ctrl vNIC, tasklet for servicing the r_vec
// @queue:          ctrl vNIC, send queue
// @lock:           ctrl vNIC, r_vec lock protects @queue
// @tx_ring:        Pointer to TX ring
// @rx_ring:        Pointer to RX ring
// @xdp_ring:	    Pointer to an extra TX ring for XDP
// @xsk_pool:	    XSK buffer pool active on vector queue pair (for AF_XDP)
// @irq_entry:      MSI-X table entry (use for talking to the device)
// @event_ctr:	    Number of interrupt
// @rx_dim:	    Dynamic interrupt moderation structure for RX
// @tx_dim:	    Dynamic interrupt moderation structure for TX
// @rx_sync:	    Seqlock for atomic updates of RX stats
// @rx_pkts:        Number of received packets
// @rx_bytes:	    Number of received bytes
// @rx_drops:	    Number of packets dropped on RX due to lack of resources
// @hw_csum_rx_ok:  Counter of packets where the HW checksum was OK
// @hw_csum_rx_inner_ok: Counter of packets where the inner HW checksum was OK
// @hw_csum_rx_complete: Counter of packets with CHECKSUM_COMPLETE reported
// @hw_csum_rx_error:	 Counter of packets with bad checksums
// @hw_tls_rx:	    Number of packets with TLS decrypted by hardware
// @tx_sync:	    Seqlock for atomic updates of TX stats
// @tx_pkts:	    Number of Transmitted packets
// @tx_bytes:	    Number of Transmitted bytes
// @hw_csum_tx:	    Counter of packets with TX checksum offload requested
// @hw_csum_tx_inner:	 Counter of inner TX checksum offload requests
// @tx_gather:	    Counter of packets with Gather DMA
// @tx_lso:	    Counter of LSO packets sent
// @hw_tls_tx:	    Counter of TLS packets sent with crypto offloaded to HW
// @tls_tx_fallback:	Counter of TLS packets sent which had to be encrypted
// by the fallback path because packets came out of order
// @tls_tx_no_fallback:	Counter of TLS packets not sent because the fallback
// path could not encrypt them
// @tx_errors:	    How many TX errors were encountered
// @tx_busy:        How often was TX busy (no space)?
// @rx_replace_buf_alloc_fail:	Counter of RX buffer allocation failures
// @irq_vector:     Interrupt vector number (use for talking to the OS)
// @handler:        Interrupt handler for this ring vector
// @name:           Name of the interrupt vector
// @affinity_mask:  SMP affinity mask for this vector
//
// This structure ties RX and TX rings to interrupt vectors and a NAPI
// context. This currently only supports one RX and TX ring per
// interrupt vector but might be extended in the future to allow
// association of multiple rings per vector.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_r_vector {
    pub nfp_net: *mut nfp_net,
    pub napi: napi_struct,
    pub tasklet: tasklet_struct,
    pub queue: sk_buff_head,
    pub lock: spinlock_t,
}

// Cold data follows
// Firmware version as it is written in the 32bit value in the BAR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_fw_version {
    pub minor: u8,
    pub major: u8,
    pub class: u8,
// This byte can be exploited for more use, currently,
// BIT0: dp type, BIT[7:1]: reserved
//
    pub extend: u8,
    pub __packed: },
    pub minor: fw_ver->minor ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_stat_pair {
    pub pkts: u64,
    pub bytes: u64,
}

//
// struct nfp_net_dp - NFP network device datapath data structure
// @dev:		Backpointer to struct device
// @netdev:		Backpointer to net_device structure
// @is_vf:		Is the driver attached to a VF?
// @chained_metadata_format:  Firemware will use new metadata format
// @ktls_tx:		Is kTLS TX enabled?
// @rx_dma_dir:		Mapping direction for RX buffers
// @rx_dma_off:		Offset at which DMA packets (for XDP headroom)
// @rx_offset:		Offset in the RX buffers where packet data starts
// @ctrl:		Local copy of the control register/word.
// @ctrl_w1:		Local copy of the control register/word1.
// @fl_bufsz:		Currently configured size of the freelist buffers
// @xdp_prog:		Installed XDP program
// @tx_rings:		Array of pre-allocated TX ring structures
// @rx_rings:		Array of pre-allocated RX ring structures
// @ctrl_bar:		Pointer to mapped control BAR
//
// @ops:		Callbacks and parameters for this vNIC's NFD version
// @txrwb:		TX pointer write back area (indexed by queue id)
// @txrwb_dma:		TX pointer write back area DMA address
// @txd_cnt:		Size of the TX ring in number of min size packets
// @rxd_cnt:		Size of the RX ring in number of min size packets
// @num_r_vecs:		Number of used ring vectors
// @num_tx_rings:	Currently configured number of TX rings
// @num_stack_tx_rings:	Number of TX rings used by the stack (not XDP)
// @num_rx_rings:	Currently configured number of RX rings
// @mtu:		Device MTU
// @xsk_pools:		XSK buffer pools, @max_r_vecs in size (for AF_XDP).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_dp {
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub is_vf:1: u8,
    pub chained_metadata_format:1: u8,
    pub ktls_tx:1: u8,
    pub rx_dma_dir: u8,
    pub rx_offset: u8,
    pub rx_dma_off: u32,
    pub ctrl: u32,
    pub ctrl_w1: u32,
    pub fl_bufsz: u32,
    pub xdp_prog: *mut bpf_prog,
    pub tx_rings: *mut nfp_net_tx_ring,
    pub rx_rings: *mut nfp_net_rx_ring,
    pub ctrl_bar: *mut u8 __iomem,
// Cold data follows
    pub ops: *const nfp_dp_ops,
    pub txrwb: *mut u64,
    pub txrwb_dma: dma_addr_t,
    pub txd_cnt: c_uint,
    pub rxd_cnt: c_uint,
    pub num_r_vecs: c_uint,
    pub num_tx_rings: c_uint,
    pub num_stack_tx_rings: c_uint,
    pub num_rx_rings: c_uint,
    pub mtu: c_uint,
    pub xsk_pools: *mut xsk_buff_pool,
}

//
// struct nfp_net - NFP network device structure
// @dp:			Datapath structure
// @dev_info:		NFP ASIC params
// @id:			vNIC id within the PF (0 for VFs)
// @fw_ver:		Firmware version
// @cap:                Capabilities advertised by the Firmware
// @cap_w1:             Extended capabilities word advertised by the Firmware
// @max_mtu:            Maximum support MTU advertised by the Firmware
// @rss_hfunc:		RSS selected hash function
// @rss_cfg:            RSS configuration
// @rss_key:            RSS secret key
// @rss_itbl:           RSS indirection table
// @xdp:		Information about the driver XDP program
// @xdp_hw:		Information about the HW XDP program
// @max_r_vecs:		Number of allocated interrupt vectors for RX/TX
// @max_tx_rings:       Maximum number of TX rings supported by the Firmware
// @max_rx_rings:       Maximum number of RX rings supported by the Firmware
// @stride_rx:		Queue controller RX queue spacing
// @stride_tx:		Queue controller TX queue spacing
// @r_vecs:             Pre-allocated array of ring vectors
// @irq_entries:        Pre-allocated array of MSI-X entries
// @lsc_handler:        Handler for Link State Change interrupt
// @lsc_name:           Name for Link State Change interrupt
// @exn_handler:        Handler for Exception interrupt
// @exn_name:           Name for Exception interrupt
// @shared_handler:     Handler for shared interrupts
// @shared_name:        Name for shared interrupt
// @reconfig_lock:	Protects @reconfig_posted, @reconfig_timer_active,
// @reconfig_sync_present and HW reconfiguration request
// regs/machinery from async requests (sync must take
// @bar_lock)
// @reconfig_posted:	Pending reconfig bits coming from async sources
// @reconfig_timer_active:  Timer for reading reconfiguration results is pending
// @reconfig_sync_present:  Some thread is performing synchronous reconfig
// @reconfig_timer:	Timer for async reading of reconfig results
// @reconfig_in_progress_update:	Update FW is processing now (debug only)
// @bar_lock:		vNIC config BAR access lock, protects: update,
// mailbox area, crypto TLV
// @link_up:            Is the link up?
// @link_status_lock:	Protects @link_* and ensures atomicity with BAR reading
// @rx_coalesce_adapt_on:   Is RX interrupt moderation adaptive?
// @tx_coalesce_adapt_on:   Is TX interrupt moderation adaptive?
// @rx_coalesce_usecs:      RX interrupt moderation usecs delay parameter
// @rx_coalesce_max_frames: RX interrupt moderation frame count parameter
// @tx_coalesce_usecs:      TX interrupt moderation usecs delay parameter
// @tx_coalesce_max_frames: TX interrupt moderation frame count parameter
// @qcp_cfg:            Pointer to QCP queue used for configuration notification
// @tx_bar:             Pointer to mapped TX queues
// @rx_bar:             Pointer to mapped FL/RX queues
// @xa_ipsec:           IPsec xarray SA data
// @tlv_caps:		Parsed TLV capabilities
// @ktls_tx_conn_cnt:	Number of offloaded kTLS TX connections
// @ktls_rx_conn_cnt:	Number of offloaded kTLS RX connections
// @ktls_conn_id_gen:	Trivial generator for kTLS connection ids (for TX)
// @ktls_no_space:	Counter of firmware rejecting kTLS connection due to
// lack of space
// @ktls_rx_resync_req:	Counter of TLS RX resync requested
// @ktls_rx_resync_ign:	Counter of TLS RX resync requests ignored
// @ktls_rx_resync_sent:    Counter of TLS RX resync completed
// @mbox_cmsg:		Common Control Message via vNIC mailbox state
// @mbox_cmsg.queue:	CCM mbox queue of pending messages
// @mbox_cmsg.wq:	CCM mbox wait queue of waiting processes
// @mbox_cmsg.workq:	CCM mbox work queue for @wait_work and @runq_work
// @mbox_cmsg.wait_work:    CCM mbox posted msg reconfig wait work
// @mbox_cmsg.runq_work:    CCM mbox posted msg queue runner work
// @mbox_cmsg.tag:	CCM mbox message tag allocator
// @debugfs_dir:	Device directory in debugfs
// @vnic_list:		Entry on device vNIC list
// @pdev:		Backpointer to PCI device
// @app:		APP handle if available
// @vnic_no_name:	For non-port PF vNIC make ndo_get_phys_port_name return
// -EOPNOTSUPP to keep backwards compatibility (set by app)
// @port:		Pointer to nfp_port structure if vNIC is a port
// @mbox_amsg:		Asynchronously processed message via mailbox
// @mbox_amsg.lock:	Protect message list
// @mbox_amsg.list:	List of message to process
// @mbox_amsg.work:	Work to process message asynchronously
// @fs:			Flow steering
// @fs.count:		Flow count
// @fs.list:		List of flows
// @app_priv:		APP private data for this vNIC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net {
    pub dp: nfp_net_dp,
    pub dev_info: *const nfp_dev_info,
    pub fw_ver: nfp_net_fw_version,
    pub id: u32,
    pub cap: u32,
    pub cap_w1: u32,
    pub max_mtu: u32,
    pub rss_hfunc: u8,
    pub rss_cfg: u32,
    pub rss_key: [u8; NFP_NET_CFG_RSS_KEY_SZ],
    pub rss_itbl: [u8; NFP_NET_CFG_RSS_ITBL_SZ],
    pub xdp: xdp_attachment_info,
    pub xdp_hw: xdp_attachment_info,
    pub max_tx_rings: c_uint,
    pub max_rx_rings: c_uint,
    pub stride_tx: c_int,
    pub stride_rx: c_int,
    pub max_r_vecs: c_uint,
    pub r_vecs: [nfp_net_r_vector; NFP_NET_MAX_R_VECS],
    pub irq_entries: [msix_entry; NFP_NET_MAX_IRQS],
    pub lsc_handler: irq_handler_t,
    pub 8]: char lsc_name[IFNAMSIZ +,
    pub exn_handler: irq_handler_t,
    pub 8]: char exn_name[IFNAMSIZ +,
    pub shared_handler: irq_handler_t,
    pub 8]: char shared_name[IFNAMSIZ +,
    pub link_up: bool,
    pub link_status_lock: spinlock_t,
    pub reconfig_lock: spinlock_t,
    pub reconfig_posted: u32,
    pub reconfig_timer_active: bool,
    pub reconfig_sync_present: bool,
    pub reconfig_timer: timer_list,
    pub reconfig_in_progress_update: u32,
    pub bar_lock: semaphore,
    pub rx_coalesce_adapt_on: bool,
    pub tx_coalesce_adapt_on: bool,
    pub rx_coalesce_usecs: u32,
    pub rx_coalesce_max_frames: u32,
    pub tx_coalesce_usecs: u32,
    pub tx_coalesce_max_frames: u32,
    pub qcp_cfg: *mut u8 __iomem,
    pub tx_bar: *mut u8 __iomem,
    pub rx_bar: *mut u8 __iomem,

    pub xa_ipsec: xarray,

    pub tlv_caps: nfp_net_tlv_caps,
    pub ktls_tx_conn_cnt: c_uint,
    pub ktls_rx_conn_cnt: c_uint,
    pub ktls_conn_id_gen: core::sync::atomic::AtomicI64,
    pub ktls_no_space: core::sync::atomic::AtomicI32,
    pub ktls_rx_resync_req: core::sync::atomic::AtomicI32,
    pub ktls_rx_resync_ign: core::sync::atomic::AtomicI32,
    pub ktls_rx_resync_sent: core::sync::atomic::AtomicI32,
    pub queue: sk_buff_head,
    pub wq: wait_queue_head_t,
    pub workq: *mut workqueue_struct,
    pub wait_work: work_struct,
    pub runq_work: work_struct,
    pub tag: u16,
    pub mbox_cmsg: },
    pub debugfs_dir: *mut dentry,
    pub vnic_list: list_head,
    pub pdev: *mut pci_dev,
    pub app: *mut nfp_app,
    pub vnic_no_name: bool,
    pub port: *mut nfp_port,
    pub lock: spinlock_t,
    pub list: list_head,
    pub work: work_struct,
    pub mbox_amsg: },
    pub count: u16,
    pub list: list_head,
    pub fs: },
    pub app_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fs_entry {
    pub node: list_head,
    pub flow_type: u32,
    pub loc: u32,
    pub sip4: __be32,
    pub dip4: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_mbox_amsg_entry {
    pub list: list_head,
    pub entry): *mut *mut *mut int (cfg)(struct nfp_net nn, struct nfp_mbox_amsg_entry,
    pub cmd: u32,
    pub msg: [c_char; ],
}

// Functions to read/write from/to a BAR
// Performs any endian conversion necessary.
//
extern "C" {
    pub fn readb(off: nn->dp.ctrl_bar +) -> return;
}
extern "C" {
    pub fn readw(off: nn->dp.ctrl_bar +) -> return;
}
extern "C" {
    pub fn readl(off: nn->dp.ctrl_bar +) -> return;
}
extern "C" {
    pub fn readq(off: nn->dp.ctrl_bar +) -> return;
}
// Flush posted PCI writes by reading something without side effects
// Queue Controller Peripheral access functions and definitions.
//
// Some of the BARs of the NFP are mapped to portions of the Queue
// Controller Peripheral (QCP) address space on the NFP.  A QCP queue
// has a read and a write pointer (as well as a size and flags,
// indicating overflow etc).  The QCP offers a number of different
// operation on queue pointers, but here we only offer function to
// either add to a pointer or to read the pointer value.
//
pub const NFP_QCP_QUEUE_ADDR_SZ: c_uint = 0x800;

pub const NFP_QCP_QUEUE_ADD_RPTR: c_uint = 0x0000;
pub const NFP_QCP_QUEUE_ADD_WPTR: c_uint = 0x0004;
pub const NFP_QCP_QUEUE_STS_LO: c_uint = 0x0008;
pub const NFP_QCP_QUEUE_STS_LO_READPTR_mask: c_uint = 0x3ffff;
pub const NFP_QCP_QUEUE_STS_HI: c_uint = 0x000c;
pub const NFP_QCP_QUEUE_STS_HI_WRITEPTR_mask: c_uint = 0x3ffff;
// nfp_qcp_ptr - Read or Write Pointer of a queue
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_qcp_ptr {
    NFP_QCP_READ_PTR = 0,
    NFP_QCP_WRITE_PTR
}

//
// nfp_qcp_rd_ptr_add() - Add the value to the read pointer of a queue
//
// @q:   Base address for queue structure
// @val: Value to add to the queue pointer
//
// nfp_qcp_wr_ptr_add() - Add the value to the write pointer of a queue
//
// @q:   Base address for queue structure
// @val: Value to add to the queue pointer
//
// nfp_qcp_rd_ptr_read() - Read the current read pointer value for a queue
// @q:  Base address for queue structure
//
// Return: Value read.
//
extern "C" {
    pub fn _nfp_qcp_read(_arg: q, _arg: NFP_QCP_READ_PTR) -> return;
}
//
// nfp_qcp_wr_ptr_read() - Read the current write pointer value for a queue
// @q:  Base address for queue structure
//
// Return: Value read.
//
extern "C" {
    pub fn _nfp_qcp_read(_arg: q, _arg: NFP_QCP_WRITE_PTR) -> return;
}
extern "C" {
    pub fn nfp_qcp_queue_offset(dev_info: *const nfp_dev_info, queue: u16) -> u32;
}
// Globals
// Prototypes
extern "C" {
    pub fn nfp_net_free(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_net_init(nn: *mut nfp_net) -> c_int;
}
extern "C" {
    pub fn nfp_net_clean(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_ctrl_open(nn: *mut nfp_net) -> c_int;
}
extern "C" {
    pub fn nfp_ctrl_close(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_net_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn nfp_net_info(nn: *mut nfp_net);
}
extern "C" {
    pub fn __nfp_net_reconfig(nn: *mut nfp_net, update: u32) -> c_int;
}
extern "C" {
    pub fn nfp_net_reconfig(nn: *mut nfp_net, update: u32) -> c_int;
}
extern "C" {
    pub fn nfp_net_rss_key_sz(nn: *mut nfp_net) -> c_uint;
}
extern "C" {
    pub fn nfp_net_rss_write_itbl(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_net_rss_write_key(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_net_coalesce_write_cfg(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_net_mbox_lock(nn: *mut nfp_net, data_size: c_uint) -> c_int;
}
extern "C" {
    pub fn nfp_net_mbox_reconfig(nn: *mut nfp_net, mbox_cmd: u32) -> c_int;
}
extern "C" {
    pub fn nfp_net_mbox_reconfig_and_unlock(nn: *mut nfp_net, mbox_cmd: u32) -> c_int;
}
extern "C" {
    pub fn nfp_net_mbox_reconfig_post(nn: *mut nfp_net, update: u32);
}
extern "C" {
    pub fn nfp_net_mbox_reconfig_wait_posted(nn: *mut nfp_net) -> c_int;
}
extern "C" {
    pub fn nfp_net_irqs_disable(pdev: *mut pci_dev);
}
extern "C" {
    pub fn nfp_net_tls_tx_undo(skb: *mut sk_buff, tls_handle: u64);
}
extern "C" {
    pub fn nfp_net_fs_add_hw(nn: *mut nfp_net, entry: *mut nfp_fs_entry) -> c_int;
}
extern "C" {
    pub fn nfp_net_fs_del_hw(nn: *mut nfp_net, entry: *mut nfp_fs_entry) -> c_int;
}

extern "C" {
    pub fn nfp_net_debugfs_create();
}
extern "C" {
    pub fn nfp_net_debugfs_destroy();
}
extern "C" {
    pub fn nfp_net_debugfs_vnic_add(nn: *mut nfp_net, ddir: *mut dentry);
}
extern "C" {
    pub fn nfp_net_debugfs_dir_clean(dir: *mut dentry);
}

