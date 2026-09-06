//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/cavium/nitrox/nitrox_dev.h
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

pub const VERSION_LEN: c_int = 32;
// Maximum queues in PF mode
pub const MAX_PF_QUEUES: c_int = 64;
// Maximum device queues

// Maximum UCD Blocks
pub const CNN55XX_MAX_UCD_BLOCKS: c_int = 8;
//
// struct nitrox_cmdq - NITROX command queue
// @cmd_qlock: command queue lock
// @resp_qlock: response queue lock
// @backlog_qlock: backlog queue lock
// @ndev: NITROX device
// @response_head: submitted request list
// @backlog_head: backlog queue
// @dbell_csr_addr: doorbell register address for this queue
// @compl_cnt_csr_addr: completion count register address of the slc port
// @base: command queue base address
// @dma: dma address of the base
// @pending_count: request pending at device
// @backlog_count: backlog request count
// @write_idx: next write index for the command
// @instr_size: command size
// @qno: command queue number
// @qsize: command queue size
// @unalign_base: unaligned base address
// @unalign_dma: unaligned dma address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_cmdq {
    pub cmd_qlock: spinlock_t,
    pub resp_qlock: spinlock_t,
    pub backlog_qlock: spinlock_t,
    pub ndev: *mut nitrox_device,
    pub response_head: list_head,
    pub backlog_head: list_head,
    pub dbell_csr_addr: *mut u8 __iomem,
    pub compl_cnt_csr_addr: *mut u8 __iomem,
    pub base: *mut u8,
    pub dma: dma_addr_t,
    pub backlog_qflush: work_struct,
    pub pending_count: core::sync::atomic::AtomicI32,
    pub backlog_count: core::sync::atomic::AtomicI32,
    pub write_idx: c_int,
    pub instr_size: u8,
    pub qno: u8,
    pub qsize: u32,
    pub unalign_base: *mut u8,
    pub unalign_dma: dma_addr_t,
}

//
// struct nitrox_hw - NITROX hardware information
// @partname: partname ex: CNN55xxx-xxx
// @fw_name: firmware version
// @freq: NITROX frequency
// @vendor_id: vendor ID
// @device_id: device ID
// @revision_id: revision ID
// @se_cores: number of symmetric cores
// @ae_cores: number of asymmetric cores
// @zip_cores: number of zip cores
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_hw {
    pub 2]: *mut *mut char partname[IFNAMSIZ,
    pub fw_name: [c_char; CNN55XX_MAX_UCD_BLOCKS][VERSION_LEN],
    pub freq: c_int,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u8,
    pub se_cores: u8,
    pub ae_cores: u8,
    pub zip_cores: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_stats {
    pub posted: core::sync::atomic::AtomicI64,
    pub completed: core::sync::atomic::AtomicI64,
    pub dropped: core::sync::atomic::AtomicI64,
}

pub const IRQ_NAMESZ: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_q_vector {
    pub name: [c_char; IRQ_NAMESZ],
    pub valid: bool,
    pub ring: c_int,
    pub resp_tasklet: tasklet_struct,
    pub cmdq: *mut nitrox_cmdq,
    pub ndev: *mut nitrox_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcode_type {
    MCODE_TYPE_INVALID,
    MCODE_TYPE_AE,
    MCODE_TYPE_SE_SSL,
    MCODE_TYPE_SE_IPSEC,
}

//
// mbox_msg - Mailbox message data
// @type: message type
// @opcode: message opcode
// @data: message data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union mbox_msg {
    pub value: u64,
    pub 2: u64 type:,
    pub 6: u64 opcode:,
    pub 58: u64 data:,
}

//
// nitrox_vfdev - NITROX VF device instance in PF
// @state: VF device state
// @vfno: VF number
// @nr_queues: number of queues enabled in VF
// @ring: ring to communicate with VF
// @msg: Mailbox message data from VF
// @mbx_resp: Mailbox counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_vfdev {
    pub state: core::sync::atomic::AtomicI32,
    pub vfno: c_int,
    pub nr_queues: c_int,
    pub ring: c_int,
    pub msg: mbox_msg,
    pub mbx_resp: core::sync::atomic::AtomicI64,
}

//
// struct nitrox_iov - SR-IOV information
// @num_vfs: number of VF(s) enabled
// @max_vf_queues: Maximum number of queues allowed for VF
// @vfdev: VF(s) devices
// @pf2vf_wq: workqueue for PF2VF communication
// @msix: MSI-X entry for PF in SR-IOV case
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_iov {
    pub num_vfs: c_int,
    pub max_vf_queues: c_int,
    pub vfdev: *mut nitrox_vfdev,
    pub pf2vf_wq: *mut workqueue_struct,
    pub msix: msix_entry,
}

//
// NITROX Device states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ndev_state {
    __NDEV_NOT_READY,
    __NDEV_READY,
    __NDEV_IN_RESET,
}

// NITROX support modes for VF(s)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vf_mode {
    __NDEV_MODE_PF,
    __NDEV_MODE_VF16,
    __NDEV_MODE_VF32,
    __NDEV_MODE_VF64,
    __NDEV_MODE_VF128,
}

pub const __NDEV_SRIOV_BIT: c_int = 0;
// command queue size
pub const DEFAULT_CMD_QLEN: c_int = 2048;
// command timeout in milliseconds
pub const CMD_TIMEOUT: c_int = 2000;

//
// struct nitrox_device - NITROX Device Information.
// @list: pointer to linked list of devices
// @bar_addr: iomap address
// @pdev: PCI device information
// @state: NITROX device state
// @flags: flags to indicate device the features
// @timeout: Request timeout in jiffies
// @refcnt: Device usage count
// @idx: device index (0..N)
// @node: NUMA node id attached
// @qlen: Command queue length
// @nr_queues: Number of command queues
// @mode: Device mode PF/VF
// @ctx_pool: DMA pool for crypto context
// @pkt_inq: Packet input rings
// @aqmq: AQM command queues
// @qvec: MSI-X queue vectors information
// @iov: SR-IOV informatin
// @num_vecs: number of MSI-X vectors
// @stats: request statistics
// @hw: hardware information
// @debugfs_dir: debugfs directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nitrox_device {
    pub list: list_head,
    pub bar_addr: *mut u8 __iomem,
    pub pdev: *mut pci_dev,
    pub state: core::sync::atomic::AtomicI32,
    pub flags: c_ulong,
    pub timeout: c_ulong,
    pub refcnt: refcount_t,
    pub idx: u8,
    pub node: c_int,
    pub qlen: u16,
    pub nr_queues: u16,
    pub mode: vf_mode,
    pub ctx_pool: *mut dma_pool,
    pub pkt_inq: *mut nitrox_cmdq,
    pub ____cacheline_aligned_in_smp: *mut *mut nitrox_cmdq aqmq[MAX_DEV_QUEUES],
    pub qvec: *mut nitrox_q_vector,
    pub iov: nitrox_iov,
    pub num_vecs: c_int,
    pub stats: nitrox_stats,
    pub hw: nitrox_hw,

    pub debugfs_dir: *mut dentry,

}

//
// nitrox_read_csr - Read from device register
// @ndev: NITROX device
// @offset: offset of the register to read
//
// Returns: value read
//
extern "C" {
    pub fn readq(offset: ndev->bar_addr +) -> return;
}
//
// nitrox_write_csr - Write to device register
// @ndev: NITROX device
// @offset: offset of the register to write
// @value: value to write
//
