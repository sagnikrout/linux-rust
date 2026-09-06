//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/fungible/funeth/funeth_txrx.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)

// Tx descriptor size

// Size of device headers per Tx packet

// Number of gather list entries per Tx descriptor

// Max gather list size in bytes for an sk_buff.

// Max number of Tx descriptors for an sk_buff using a gather list.

// Max number of Tx descriptors for any packet.

// Rx CQ descriptor size.

// Offset of cqe_info within a CQE.

// Construct the IRQ portion of a CQ doorbell. The resulting value arms the
// interrupt with the supplied time delay and packet count moderation settings.
//

// As above for SQ doorbells.

// Per packet tailroom. Present only for 1-frag packets.

// Per packet headroom for XDP. Preferred over XDP_PACKET_HEADROOM to
// accommodate two packets per buffer for 4K pages and 1500B MTUs.
//
pub const FUN_XDP_HEADROOM: c_int = 192;
// Initialization state of a queue.
// Initialization state of an interrupt.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct funeth_txq_stats {
    pub /: *mut *mut u64 tx_pkts; / # of Tx packets,
    pub /: *mut *mut u64 tx_bytes; / total bytes of Tx packets,
    pub /: *mut *mut u64 tx_cso; / # of packets with checksum offload,
    pub /: *mut *mut u64 tx_tso; / # of non-encapsulated TSO super-packets,
    pub /: *mut *mut u64 tx_encap_tso; / # of encapsulated TSO super-packets,
    pub /: *mut *mut u64 tx_uso; / # of non-encapsulated UDP LSO super-packets,
    pub /: *mut *mut u64 tx_more; / # of DBs elided due to xmit_more,
    pub /: *mut *mut u64 tx_nstops; / # of times the queue has stopped,
    pub /: *mut *mut u64 tx_nrestarts; / # of times the queue has restarted,
    pub /: *mut *mut u64 tx_map_err; / # of packets dropped due to DMA mapping errors,
    pub /: *mut *mut u64 tx_xdp_full; / # of XDP packets that could not be enqueued,
    pub /: *mut *mut u64 tx_tls_pkts; / # of Tx TLS packets offloaded to HW,
    pub /: *mut *mut u64 tx_tls_bytes; / Tx bytes of HW-handled TLS payload,
    pub /: *mut *mut u64 tx_tls_fallback; / attempted Tx TLS offloads punted to SW,
    pub /: *mut *mut u64 tx_tls_drops; / attempted Tx TLS offloads dropped,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct funeth_tx_info {
    pub /: *mut *mut *mut sk_buff skb; / associated packet (sk_buff path),
    pub /: *mut *mut *mut xdp_frame xdpf; / associated XDP frame (XDP path),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct funeth_txq {
// RO cacheline of frequently accessed data
    pub /: *mut *mut u32 mask; / queue depth - 1,
    pub /: *mut *mut u32 hw_qid; / device ID of the queue,
    pub /: *mut *mut *mut void desc; / base address of descriptor ring,
    pub info: *mut funeth_tx_info,
    pub /: *mut *mut *mut device dma_dev; / device for DMA mappings,
    pub /: *mut *mut *mut volatile __be64 hw_wb; / HW write-back location,
    pub /: *mut *mut *mut u32 __iomem db; / SQ doorbell register address,
    pub ndq: *mut netdev_queue,
    pub /: *mut *mut dma_addr_t dma_addr; / DMA address of descriptor ring,
// producer R/W cacheline
    pub /: *mut *mut u16 qidx; / queue index within net_device,
    pub ethid: u16,
    pub /: *mut *mut u32 prod_cnt; / producer counter,
    pub stats: funeth_txq_stats,
// shared R/W cacheline, primarily accessed by consumer
    pub /: *mut *mut u32 irq_db_val; / value written to IRQ doorbell,
    pub /: *mut *mut u32 cons_cnt; / consumer (cleanup) counter,
    pub netdev: *mut net_device,
    pub irq: *mut fun_irq,
    pub numa_node: c_int,
    pub /: *mut *mut u8 init_state; / queue initialization state,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct funeth_rxq_stats {
    pub /: *mut *mut u64 rx_pkts; / # of received packets, including SW drops,
    pub /: *mut *mut u64 rx_bytes; / total size of received packets,
    pub /: *mut *mut u64 rx_cso; / # of packets with checksum offload,
    pub /: *mut *mut u64 rx_bufs; / total # of Rx buffers provided to device,
    pub /: *mut *mut u64 gro_pkts; / # of GRO superpackets,
    pub /: *mut *mut u64 gro_merged; / # of pkts merged into existing GRO superpackets,
    pub /: *mut *mut u64 rx_page_alloc; / # of page allocations for Rx buffers,
    pub /: *mut *mut u64 rx_budget; / NAPI iterations that exhausted their budget,
    pub /: *mut *mut u64 rx_mem_drops; / # of packets dropped due to memory shortage,
    pub /: *mut *mut u64 rx_map_err; / # of page DMA mapping errors,
    pub /: *mut *mut u64 xdp_drops; / XDP_DROPped packets,
    pub /: *mut *mut u64 xdp_tx; / successful XDP transmits,
    pub /: *mut *mut u64 xdp_redir; / successful XDP redirects,
    pub /: *mut *mut u64 xdp_err; / packets dropped due to XDP errors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct funeth_rxbuf {
    pub /: *mut *mut *mut page page; / associated page,
    pub /: *mut *mut dma_addr_t dma_addr; / DMA address of page start,
    pub /: *mut *mut int pg_refs; / page refs held by driver,
    pub /: *mut *mut int node; / page node, or -1 if it is PF_MEMALLOC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct funeth_rx_cache {
    pub /: *mut *mut *mut funeth_rxbuf bufs; / base of Rx buffer state ring,
    pub /: *mut *mut unsigned int prod_cnt; / producer counter,
    pub /: *mut *mut unsigned int cons_cnt; / consumer counter,
    pub /: *mut *mut unsigned int mask; / depth - 1,
}

// An Rx queue consists of a CQ and an SQ used to provide Rx buffers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct funeth_rxq {
    pub netdev: *mut net_device,
    pub napi: *mut napi_struct,
    pub /: *mut *mut *mut device dma_dev; / device for DMA mappings,
    pub /: *mut *mut *mut void cqes; / base of CQ descriptor ring,
    pub /: *const *const *const void next_cqe_info; / fun_cqe_info of next CQE,
    pub /: *mut *mut *mut u32 __iomem cq_db; / CQ doorbell register address,
    pub /: *mut *mut unsigned int cq_head; / CQ head index,
    pub /: *mut *mut unsigned int cq_mask; / CQ depth - 1,
    pub /: *mut *mut u16 phase; / CQ phase tag,
    pub /: *mut *mut u16 qidx; / queue index within net_device,
    pub /: *mut *mut unsigned int irq_db_val; / IRQ info for CQ doorbell,
    pub /: *mut *mut *mut fun_eprq_rqbuf rqes; / base of RQ descriptor ring,
    pub /: *mut *mut *mut funeth_rxbuf bufs; / base of Rx buffer state ring,
    pub /: *mut *mut *mut funeth_rxbuf cur_buf; / currently active buffer,
    pub /: *mut *mut *mut u32 __iomem rq_db; / RQ doorbell register address,
    pub /: *mut *mut unsigned int rq_cons; / RQ consumer counter,
    pub /: *mut *mut unsigned int rq_mask; / RQ depth - 1,
    pub /: *mut *mut unsigned int buf_offset; / offset of next pkt in head buffer,
    pub /: *mut *mut u8 xdp_flush; / XDP flush types needed at NAPI end,
    pub /: *mut *mut u8 init_state; / queue initialization state,
    pub /: *mut *mut u16 headroom; / per packet headroom,
    pub /: *mut *mut unsigned int rq_cons_db; / value of rq_cons at last RQ db,
    pub /: *mut *mut unsigned int rq_db_thres; / # of new buffers needed to write RQ db,
    pub /: *mut *mut funeth_rxbuf spare_buf; / spare for next buffer replacement,
    pub /: *mut *mut funeth_rx_cache cache; / used buffer cache,
    pub /: *mut *mut *mut bpf_prog xdp_prog; / optional XDP BPF program,
    pub stats: funeth_rxq_stats,
    pub /: *mut *mut dma_addr_t cq_dma_addr; / DMA address of CQE ring,
    pub /: *mut *mut dma_addr_t rq_dma_addr; / DMA address of RQE ring,
    pub irq_cnt: u16,
    pub /: *mut *mut u32 hw_cqid; / device ID of the queue's CQ,
    pub /: *mut *mut u32 hw_sqid; / device ID of the queue's SQ,
    pub numa_node: c_int,
    pub syncp: u64_stats_sync,
    pub xdp_rxq: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_irq {
    pub napi: napi_struct,
    pub txq: *mut funeth_txq,
    pub rxq: *mut funeth_rxq,
    pub state: u8,
    pub /: *mut *mut u16 irq_idx; / index of MSI-X interrupt,
    pub /: *mut *mut int irq; / Linux IRQ vector,
    pub /: *mut *mut cpumask_t affinity_mask; / IRQ affinity,
    pub aff_notify: irq_affinity_notify,
    pub name: [c_char; FUN_INT_NAME_LEN],
    pub ____cacheline_internodealigned_in_smp: },
// Return the start address of the idx-th Tx descriptor.
    pub FUNETH_SQE_SIZE: *mut *mut return q->desc + idx,
    pub q->mask: unsigned int tail = q->prod_cnt &,
    pub q->db): writel(tail,,
    pub cpu_to_mem(cpumask_first(&p->affinity_mask)): return,
    pub budget): *mut *mut int fun_rxq_napi_poll(struct napi_struct napi, int,
    pub budget): *mut *mut int fun_txq_napi_poll(struct napi_struct napi, int,
    pub netdev): *mut *mut netdev_tx_t fun_start_xmit(struct sk_buff skb, struct net_device,
    pub xdpf): *mut *mut bool fun_xdp_tx(struct funeth_txq q, struct xdp_frame,
    pub flags): *mut *mut *mut xdp_frame frames, u32,
    pub qp): *mut funeth_txq,
    pub irq): *mut *mut int fun_txq_create_dev(struct funeth_txq q, struct fun_irq,
    pub state): *mut *mut *mut funeth_txq funeth_txq_free(funeth_txq q, int,
    pub qp): *mut int state, struct funeth_rxq,
    pub irq): *mut *mut int fun_rxq_create_dev(struct funeth_rxq q, struct fun_irq,
    pub state): *mut *mut *mut funeth_rxq funeth_rxq_free(funeth_rxq q, int,
    pub prog): *mut *mut int fun_rxq_set_bpf(struct funeth_rxq q, struct bpf_prog,
