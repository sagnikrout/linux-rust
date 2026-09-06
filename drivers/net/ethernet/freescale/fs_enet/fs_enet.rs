//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fs_enet/fs_enet.h
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

// MPC5121 FEC has different register layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec {
    pub fec_reserved0: u32,
    pub /: *mut *mut u32 fec_ievent; / Interrupt event reg,
    pub /: *mut *mut u32 fec_imask; / Interrupt mask reg,
    pub fec_reserved1: u32,
    pub /: *mut *mut u32 fec_r_des_active; / Receive descriptor reg,
    pub /: *mut *mut u32 fec_x_des_active; / Transmit descriptor reg,
    pub fec_reserved2: [u32; 3],
    pub /: *mut *mut u32 fec_ecntrl; / Ethernet control reg,
    pub fec_reserved3: [u32; 6],
    pub /: *mut *mut u32 fec_mii_data; / MII manage frame reg,
    pub /: *mut *mut u32 fec_mii_speed; / MII speed control reg,
    pub fec_reserved4: [u32; 7],
    pub /: *mut *mut u32 fec_mib_ctrlstat; / MIB control/status reg,
    pub fec_reserved5: [u32; 7],
    pub /: *mut *mut u32 fec_r_cntrl; / Receive control reg,
    pub fec_reserved6: [u32; 15],
    pub /: *mut *mut u32 fec_x_cntrl; / Transmit Control reg,
    pub fec_reserved7: [u32; 7],
    pub /: *mut *mut u32 fec_addr_low; / Low 32bits MAC address,
    pub /: *mut *mut u32 fec_addr_high; / High 16bits MAC address,
    pub /: *mut *mut u32 fec_opd; / Opcode + Pause duration,
    pub fec_reserved8: [u32; 10],
    pub /: *mut *mut u32 fec_hash_table_high; / High 32bits hash table,
    pub /: *mut *mut u32 fec_hash_table_low; / Low 32bits hash table,
    pub /: *mut *mut u32 fec_grp_hash_table_high; / High 32bits hash table,
    pub /: *mut *mut u32 fec_grp_hash_table_low; / Low 32bits hash table,
    pub fec_reserved9: [u32; 7],
    pub /: *mut *mut u32 fec_x_wmrk; / FIFO transmit water mark,
    pub fec_reserved10: u32,
    pub /: *mut *mut u32 fec_r_bound; / FIFO receive bound reg,
    pub /: *mut *mut u32 fec_r_fstart; / FIFO receive start reg,
    pub fec_reserved11: [u32; 11],
    pub /: *mut *mut u32 fec_r_des_start; / Receive descriptor ring,
    pub /: *mut *mut u32 fec_x_des_start; / Transmit descriptor ring,
    pub /: *mut *mut u32 fec_r_buff_size; / Maximum receive buff size,
    pub fec_reserved12: [u32; 26],
    pub /: *mut *mut u32 fec_dma_control; / DMA Endian and other ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec_info {
    pub fecp: *mut fec __iomem,
    pub mii_speed: u32,
}

// hw driver ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_ops {
    pub dev): *mut *mut int (setup_data)(struct net_device,
    pub dev): *mut *mut int (allocate_bd)(struct net_device,
    pub dev): *mut *mut void (free_bd)(struct net_device,
    pub dev): *mut *mut void (cleanup_data)(struct net_device,
    pub dev): *mut *mut void (set_multicast_list)(struct net_device,
    pub duplex): int speed, int,
    pub dev): *mut *mut void (stop)(struct net_device,
    pub dev): *mut *mut void (napi_clear_event)(struct net_device,
    pub dev): *mut *mut void (napi_enable)(struct net_device,
    pub dev): *mut *mut void (napi_disable)(struct net_device,
    pub dev): *mut *mut void (rx_bd_done)(struct net_device,
    pub dev): *mut *mut void (tx_kickstart)(struct net_device,
    pub dev): *mut *mut u32 (get_int_events)(struct net_device,
    pub int_events): *mut *mut *mut void (clear_int_events)(struct net_device dev, u32,
    pub int_events): *mut *mut *mut void (ev_error)(struct net_device dev, u32,
    pub sizep): *mut *mut *mut *mut int (get_regs)(struct net_device dev, void p, int,
    pub dev): *mut *mut int (get_regs_len)(struct net_device,
    pub dev): *mut *mut void (tx_restart)(struct net_device,
}

// The FEC stores dest/src/type, data, and checksum for receive packets.
//

pub const CRC_LEN: c_int = 4;

// Must be a multiple of 32 (to cover both FEC & FCC)

// This is needed so that invalidate_xxx wont invalidate too much
pub const ENET_RX_ALIGN: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_platform_info {
// device specific information
    pub /: *mut *mut u32 cp_command; / CPM page/sblock/mcn,
    pub dpram_offset: u32,
    pub /: *mut *mut int rx_ring, tx_ring; / number of buffers on rx,
    pub /: *mut *mut int rx_copybreak; / limit we copy small frames,
    pub /: *mut *mut int napi_weight; / NAPI weight,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_enet_private {
    pub napi: napi_struct,
    pub /: *mut *mut *mut device dev; / pointer back to the device (must be initialized first),
    pub ndev: *mut net_device,
    pub /: *mut *mut spinlock_t lock; / during all ops except TX pckt processing,
    pub /: *mut *mut spinlock_t tx_lock; / during fs_start_xmit and fs_tx,
    pub fpi: *mut fs_platform_info,
    pub timeout_work: work_struct,
    pub ops: *const fs_ops,
    pub tx_ring: int rx_ring,,
    pub ring_mem_addr: dma_addr_t,
    pub ring_base: *mut void __iomem,
    pub rx_skbuff: *mut sk_buff,
    pub tx_skbuff: *mut sk_buff,
    pub mapped_as_page: *mut c_char,
    pub /: *mut *mut *mut cbd_t __iomem rx_bd_base; / Address of Rx and Tx buffers.,
    pub tx_bd_base: *mut cbd_t __iomem,
    pub /: *mut *mut *mut cbd_t __iomem dirty_tx; / ring entries to be free()ed.,
    pub cur_rx: *mut cbd_t __iomem,
    pub cur_tx: *mut cbd_t __iomem,
    pub tx_free: c_int,
    pub msg_enable: u32,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub interrupt: c_int,
// event masks
    pub /: *mut *mut u32 ev_napi; / mask of NAPI events,
    pub /: *mut *mut u32 ev; / event mask,
    pub /: *mut *mut u32 ev_err; / error event mask,
    pub /: *mut *mut u16 bd_rx_empty; / mask of BD rx empty,
    pub /: *mut *mut u16 bd_rx_err; / mask of BD rx errors,
    pub /: *mut *mut int idx; / FEC1 = 0, FEC2 = 1,
    pub /: *mut *mut *mut void __iomem fecp; / hw registers,
    pub /: *mut *mut u32 hthi, htlo; / state for multicast,
    pub fec: },
    pub /: *mut *mut int idx; / FCC1-3 = 0-2,
    pub /: *mut *mut *mut void __iomem fccp; / hw registers,
    pub /: *mut *mut *mut void __iomem ep; / parameter ram,
    pub /: *mut *mut *mut void __iomem fcccp; / hw registers cont.,
    pub /: *mut *mut *mut void __iomem mem; / FCC DPRAM,
    pub /: *mut *mut u32 gaddrh, gaddrl; / group address,
    pub fcc: },
    pub /: *mut *mut int idx; / FEC1 = 0, FEC2 = 1,
    pub /: *mut *mut *mut void __iomem sccp; / hw registers,
    pub /: *mut *mut *mut void __iomem ep; / parameter ram,
    pub /: *mut *mut u32 hthi, htlo; / state for multicast,
    pub scc: },
}

//
extern "C" {
    pub fn fs_init_bds(dev: *mut net_device);
}
extern "C" {
    pub fn fs_cleanup_bds(dev: *mut net_device);
}
//

//
// buffer descriptor access macros
// access macros

// for a CPM1 __raw_xxx's are sufficient

// for others play it safe

// write

// read

// set bits

// clear bits

//
