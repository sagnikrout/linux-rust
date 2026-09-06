//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/hfi.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2020-2023 Cornelis Networks, Inc.
// Copyright(c) 2015-2020 Intel Corporation.
//

// bumped 1 from s/w major version of TrueScale

// don't care about this except printing

// The Organization Unique Identifier (Mfg code), and its position in GUID
pub const HFI1_OUI: c_uint = 0x001175;
pub const HFI1_OUI_LSB: c_int = 40;
pub const DROP_PACKET_OFF: c_int = 0;
pub const DROP_PACKET_ON: c_int = 1;
pub const NEIGHBOR_TYPE_HFI: c_int = 0;
pub const NEIGHBOR_TYPE_SWITCH: c_int = 1;
pub const HFI1_MAX_ACTIVE_WORKQUEUE_ENTRIES: c_int = 5;

// Offline Disabled Reason is 4-bits

//
// Control context is always 0 and handles the error packets.
// It also handles the VL15 and multicast packets.
//
pub const HFI1_CTRL_CTXT: c_int = 0;
//
// Driver context will store software counters for each of the events
// associated with these status registers
//
pub const NUM_CCE_ERR_STATUS_COUNTERS: c_int = 41;
pub const NUM_RCV_ERR_STATUS_COUNTERS: c_int = 64;
pub const NUM_MISC_ERR_STATUS_COUNTERS: c_int = 13;
pub const NUM_SEND_PIO_ERR_STATUS_COUNTERS: c_int = 36;
pub const NUM_SEND_DMA_ERR_STATUS_COUNTERS: c_int = 4;
pub const NUM_SEND_EGRESS_ERR_STATUS_COUNTERS: c_int = 64;
pub const NUM_SEND_ERR_STATUS_COUNTERS: c_int = 3;
pub const NUM_SEND_CTXT_ERR_STATUS_COUNTERS: c_int = 5;
pub const NUM_SEND_DMA_ENG_ERR_STATUS_COUNTERS: c_int = 24;
//
// per driver stats, either not device nor port-specific, or
// summed over all of the devices and ports.
// They are described by name via ipathfs filesystem, so layout
// and number of elements can change without breaking compatibility.
// If members are added or deleted hfi1_statnames[] in debugfs.c must
// change to match.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ib_stats {
    pub /: *mut *mut __u64 sps_ints; / number of interrupts handled,
    pub /: *mut *mut __u64 sps_errints; / number of error interrupts,
    pub /: *mut *mut __u64 sps_txerrs; / tx-related packet errors,
    pub /: *mut *mut __u64 sps_rcverrs; / non-crc rcv packet errors,
    pub /: *mut *mut __u64 sps_hwerrs; / hardware errors reported (parity, etc.),
    pub /: *mut *mut __u64 sps_nopiobufs; / no pio bufs avail from kernel,
    pub /: *mut *mut __u64 sps_ctxts; / number of contexts currently open,
    pub /: *mut *mut __u64 sps_lenerrs; / number of kernel packets where RHF != LRH len,
    pub sps_buffull: __u64,
    pub sps_hdrfull: __u64,
}

//
// First-cut criterion for "device is active" is
// two thousand dwords combined Tx, Rx traffic per
// 5-second interval. SMA packets are 64 dwords,
// and occur "a few per second", presumably each way.
//

//
// Below contains all data related to a single context (formerly called port).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctxt_eager_bufs {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eager_buffer {
    pub addr: *mut c_void,
    pub dma: dma_addr_t,
    pub len: isize,
    pub buffers: *mut },
    pub addr: *mut c_void,
    pub dma: dma_addr_t,
    pub rcvtids: *mut },
    pub /: *mut *mut u32 size; / total size of eager buffers,
    pub /: *mut *mut u32 rcvtid_size; / size of each eager rcv tid,
    pub /: *mut *mut u16 count; / size of buffers array,
    pub /: *mut *mut u16 numbufs; / number of buffers allocated,
    pub /: *mut *mut u16 alloced; / number of rcvarray entries used,
    pub /: *mut *mut u16 threshold; / head update threshold,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exp_tid_set {
    pub list: list_head,
    pub count: u32,
}

extern "C" {
    pub fn int(rcd: *mut *mut intr_handler)(struct hfi1_ctxtdata, data: c_int) -> typedef;
}
extern "C" {
    pub fn void(packet: *mut *mut rhf_rcv_function_ptr)(struct hfi1_packet) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_queue {
    pub queue_head: list_head,
// queue head for QP TID resource waiters
    pub /: *mut *mut u32 enqueue; / count of tid enqueues,
    pub /: *mut *mut u32 dequeue; / count of tid dequeues,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ctxtdata {
// rcvhdrq base, needs mmap before useful
    pub rcvhdrq: *mut c_void,
// kernel virtual address where hdrqtail is updated
    pub rcvhdrtail_kvaddr: *mut volatile __le64,
// so functions that need physical port can get it easily
    pub ppd: *mut hfi1_pportdata,
// so file ops can get at unit
    pub dd: *mut hfi1_devdata,
// this receive context's assigned PIO ACK send context
    pub sc: *mut send_context,
// per context recv functions
    pub rhf_rcv_function_map: *const rhf_rcv_function_ptr,
//
// The interrupt handler for a particular receive context can vary
// throughout it's lifetime. This is not a lock protected data member so
// it must be updated atomically and the prev and new value must always
// be valid. Worst case is we process an extra interrupt and up to 64
// packets with the wrong interrupt handler.
//
    pub do_interrupt: intr_handler,
// fast handler after autoactive
    pub fast_handler: intr_handler,
// slow handler
    pub slow_handler: intr_handler,
// napi pointer assiociated with netdev
    pub napi: *mut napi_struct,
// verbs rx_stats per rcd
    pub opstats: *mut hfi1_opcode_stats_perctx,
// clear interrupt mask
    pub imask: u64,
// ctxt rcvhdrq head offset
    pub head: u32,
// number of rcvhdrq entries
    pub rcvhdrq_cnt: u16,
    pub /: *mut *mut u8 ireg; / clear interrupt register,
// receive packet sequence counter
    pub seq_cnt: u8,
// size of each of the rcvhdrq entries
    pub rcvhdrqentsize: u8,
// offset of RHF within receive header entry
    pub rhf_offset: u8,
// dynamic receive available interrupt timeout
    pub rcvavail_timeout: u8,
// Is ASPM interrupt supported for this context
    pub aspm_intr_supported: bool,
// ASPM state (enabled/disabled) for this context
    pub aspm_enabled: bool,
// Is ASPM processing enabled for this context (in intr context)
    pub aspm_intr_enable: bool,
    pub egrbufs: ctxt_eager_bufs,
// QPs waiting for context processing
    pub qp_wait_list: list_head,
// tid allocation lists
    pub tid_group_list: exp_tid_set,
    pub tid_used_list: exp_tid_set,
    pub tid_full_list: exp_tid_set,
// Timer for re-enabling ASPM if interrupt activity quiets down
    pub aspm_timer: timer_list,
// per-context configuration flags
    pub flags: c_ulong,
// array of tid_groups
    pub groups: *mut tid_group,
// mmap of hdrq, must fit in 44 bits
    pub rcvhdrq_dma: dma_addr_t,
    pub rcvhdrqtailaddr_dma: dma_addr_t,
// Last interrupt timestamp
    pub aspm_ts_last_intr: ktime_t,
// Last timestamp at which we scheduled a timer for this context
    pub aspm_ts_timer_sched: ktime_t,
// Lock to serialize between intr, timer intr and user threads
    pub aspm_lock: spinlock_t,
// Reference count the base context usage
    pub kref: kref,
// numa node of this context
    pub numa_id: c_int,
// associated msix interrupt.
    pub msix_intr: i16,
// job key
    pub jkey: u16,
// number of RcvArray groups for this context.
    pub rcv_array_groups: u16,
// index of first eager TID entry.
    pub eager_base: u16,
// number of expected TID entries
    pub expected_count: u16,
// index of first expected TID entry.
    pub expected_base: u16,
// Device context index
    pub ctxt: u8,
// PSM Specific fields
// lock protecting all Expected TID data
    pub exp_mutex: mutex,
// lock protecting all Expected TID data of kernel contexts
    pub exp_lock: spinlock_t,
// Queue for QP's waiting for HW TID flows
    pub flow_queue: tid_queue,
// Queue for QP's waiting for HW receive array entries
    pub rarr_queue: tid_queue,
// when waiting for rcv or pioavail
    pub wait: wait_queue_head_t,
// uuid from PSM
    pub uuid: [u8; 16],
// same size as task_struct .comm[], command that opened context
    pub comm: [c_char; TASK_COMM_LEN],
// Bitmask of in use context(s)
    pub HFI1_MAX_SHARED_CTXTS): DECLARE_BITMAP(in_use_ctxts,,
// per-context event flags for fileops/intr communication
    pub event_flags: c_ulong,
// A page of memory for rcvhdrhead, rcvegrhead, rcvegrtail * N
    pub subctxt_uregbase: *mut c_void,
// An array of pages for the eager receive buffers * N
    pub subctxt_rcvegrbuf: *mut c_void,
// An array of pages for the eager header queue entries * N
    pub subctxt_rcvhdr_base: *mut c_void,
// total number of polled urgent packets
    pub urgent: u32,
// saved total number of polled urgent packets for poll edge trigger
    pub urgent_poll: u32,
// Type of packets or conditions we want to poll for
    pub poll_type: u16,
// non-zero if ctxt is being shared.
    pub subctxt_id: u16,
// The version of the library which opened this ctxt
    pub userversion: u32,
//
// non-zero if ctxt can be shared, and defines the maximum number of
// sub-contexts for this device context.
//
    pub subctxt_cnt: u8,
// Bit mask to track free TID RDMA HW flows
    pub flow_mask: c_ulong,
    pub flows: [tid_flow_state; RXE_NUM_TID_FLOWS],
}

//
// rcvhdrq_size - return total size in bytes for header queue
// @rcd: the receive context
//
// rcvhdrqentsize is in DWs, so we have to convert to bytes
//
// Represents a single packet at a high level. Put commonly computed things in
// here so we do not have to keep doing them over and over. The rule of thumb is
// if something is used one time to derive some value, store that something in
// here. If it is used multiple times, then store the result of that derivation
// in here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_packet {
    pub ebuf: *mut c_void,
    pub hdr: *mut c_void,
    pub payload: *mut c_void,
    pub rcd: *mut hfi1_ctxtdata,
    pub rhf_addr: *mut __le32,
    pub qp: *mut rvt_qp,
    pub ohdr: *mut ib_other_headers,
    pub grh: *mut ib_grh,
    pub mgmt: *mut opa_16b_mgmt,
    pub rhf: u64,
    pub maxcnt: u32,
    pub rhqoff: u32,
    pub dlid: u32,
    pub slid: u32,
    pub numpkt: c_int,
    pub tlen: u16,
    pub etail: i16,
    pub pkey: u16,
    pub hlen: u8,
    pub rsize: u8,
    pub updegr: u8,
    pub etype: u8,
    pub extra_byte: u8,
    pub pad: u8,
    pub sc: u8,
    pub sl: u8,
    pub opcode: u8,
    pub migrated: bool,
}

// Packet types
pub const HFI1_PKT_TYPE_9B: c_int = 0;
pub const HFI1_PKT_TYPE_16B: c_int = 1;
//
// OPA 16B Header
//
pub const OPA_16B_L4_MASK: c_uint = 0xFFull;
pub const OPA_16B_SC_MASK: c_uint = 0x1F00000ull;
pub const OPA_16B_SC_SHIFT: c_int = 20;
pub const OPA_16B_LID_MASK: c_uint = 0xFFFFFull;
pub const OPA_16B_DLID_MASK: c_uint = 0xF000ull;
pub const OPA_16B_DLID_SHIFT: c_int = 20;
pub const OPA_16B_DLID_HIGH_SHIFT: c_int = 12;
pub const OPA_16B_SLID_MASK: c_uint = 0xF00ull;
pub const OPA_16B_SLID_SHIFT: c_int = 20;
pub const OPA_16B_SLID_HIGH_SHIFT: c_int = 8;
pub const OPA_16B_BECN_MASK: c_uint = 0x80000000ull;
pub const OPA_16B_BECN_SHIFT: c_int = 31;
pub const OPA_16B_FECN_MASK: c_uint = 0x10000000ull;
pub const OPA_16B_FECN_SHIFT: c_int = 28;
pub const OPA_16B_L2_MASK: c_uint = 0x60000000ull;
pub const OPA_16B_L2_SHIFT: c_int = 29;
pub const OPA_16B_PKEY_MASK: c_uint = 0xFFFF0000ull;
pub const OPA_16B_PKEY_SHIFT: c_int = 16;
pub const OPA_16B_LEN_MASK: c_uint = 0x7FF00000ull;
pub const OPA_16B_LEN_SHIFT: c_int = 20;
pub const OPA_16B_RC_MASK: c_uint = 0xE000000ull;
pub const OPA_16B_RC_SHIFT: c_int = 25;
pub const OPA_16B_AGE_MASK: c_uint = 0xFF0000ull;
pub const OPA_16B_AGE_SHIFT: c_int = 16;
pub const OPA_16B_ENTROPY_MASK: c_uint = 0xFFFFull;
//
// OPA 16B L2/L4 Encodings
//
pub const OPA_16B_L4_9B: c_uint = 0x00;
pub const OPA_16B_L2_TYPE: c_uint = 0x02;
pub const OPA_16B_L4_FM: c_uint = 0x08;
pub const OPA_16B_L4_IB_LOCAL: c_uint = 0x09;
pub const OPA_16B_L4_IB_GLOBAL: c_uint = 0x0A;
//
// OPA 16B Management
//

//
// BTH
//
pub const OPA_16B_BTH_PAD_MASK: c_int = 7;
//
// 16B Management
//
pub const OPA_16B_MGMT_QPN_MASK: c_uint = 0xFFFFFF;
//
// hfi1_get_rc_ohdr - get extended header
// @opah - the opaheader
//
// Find out where the BTH is
//
// Get/Set IB link-level config parameters for f_get/set_ib_cfg()
// Mostly for MADs that set or query link parameters, also ipath
// config interfaces
//

pub const HFI1_IB_CFG_VL_HIGH_LIMIT: c_int = 19;

//
// HFI or Host Link States
//
// These describe the states the driver thinks the logical and physical
// states are in.  Used as an argument to set_link_state().  Implemented
// as bits for easy multi-state checking.  The actual state can only be
// one.
//
pub const __HLS_UP_INIT_BP: c_int = 0;
pub const __HLS_UP_ARMED_BP: c_int = 1;
pub const __HLS_UP_ACTIVE_BP: c_int = 2;

pub const __HLS_DN_POLL_BP: c_int = 4;
pub const __HLS_DN_DISABLE_BP: c_int = 5;
pub const __HLS_DN_OFFLINE_BP: c_int = 6;
pub const __HLS_VERIFY_CAP_BP: c_int = 7;
pub const __HLS_GOING_UP_BP: c_int = 8;
pub const __HLS_GOING_OFFLINE_BP: c_int = 9;
pub const __HLS_LINK_COOLDOWN_BP: c_int = 10;

// use this MTU size if none other is given
pub const HFI1_DEFAULT_ACTIVE_MTU: c_int = 10240;
// use this MTU size as the default maximum
pub const HFI1_DEFAULT_MAX_MTU: c_int = 10240;
// default partition key
pub const DEFAULT_PKEY: c_uint = 0xffff;
//
// Possible fabric manager config parameters for fm_{get,set}_table()
//

//
// Possible "operations" for f_rcvctrl(ppd, op, ctxt)
// these are bits so they can be combined, e.g.
// HFI1_RCVCTRL_INTRAVAIL_ENB | HFI1_RCVCTRL_CTXT_ENB
//
pub const HFI1_RCVCTRL_TAILUPD_ENB: c_uint = 0x01;
pub const HFI1_RCVCTRL_TAILUPD_DIS: c_uint = 0x02;
pub const HFI1_RCVCTRL_CTXT_ENB: c_uint = 0x04;
pub const HFI1_RCVCTRL_CTXT_DIS: c_uint = 0x08;
pub const HFI1_RCVCTRL_INTRAVAIL_ENB: c_uint = 0x10;
pub const HFI1_RCVCTRL_INTRAVAIL_DIS: c_uint = 0x20;
pub const HFI1_RCVCTRL_PKEY_ENB: c_uint = 0x40  /* Note, default is enabled */;
pub const HFI1_RCVCTRL_PKEY_DIS: c_uint = 0x80;
pub const HFI1_RCVCTRL_TIDFLOW_ENB: c_uint = 0x0400;
pub const HFI1_RCVCTRL_TIDFLOW_DIS: c_uint = 0x0800;
pub const HFI1_RCVCTRL_ONE_PKT_EGR_ENB: c_uint = 0x1000;
pub const HFI1_RCVCTRL_ONE_PKT_EGR_DIS: c_uint = 0x2000;
pub const HFI1_RCVCTRL_NO_RHQ_DROP_ENB: c_uint = 0x4000;
pub const HFI1_RCVCTRL_NO_RHQ_DROP_DIS: c_uint = 0x8000;
pub const HFI1_RCVCTRL_NO_EGR_DROP_ENB: c_uint = 0x10000;
pub const HFI1_RCVCTRL_NO_EGR_DROP_DIS: c_uint = 0x20000;
pub const HFI1_RCVCTRL_URGENT_ENB: c_uint = 0x40000;
pub const HFI1_RCVCTRL_URGENT_DIS: c_uint = 0x80000;
// partition enforcement flags
pub const HFI1_PART_ENFORCE_IN: c_uint = 0x1;
pub const HFI1_PART_ENFORCE_OUT: c_uint = 0x2;
// how often we check for synthetic counter wrap around
pub const SYNTH_CNT_TIME: c_int = 3;
// Counter flags
pub const CNTR_NORMAL: c_uint = 0x0 /* Normal counters, just read register */;
pub const CNTR_SYNTH: c_uint = 0x1 /* Synthetic counters, saturate at all 1s */;
pub const CNTR_DISABLED: c_uint = 0x2 /* Disable this counter */;
pub const CNTR_32BIT: c_uint = 0x4 /* Simulate 64 bits for this counter */;
pub const CNTR_VL: c_uint = 0x8 /* Per VL counter */;
pub const CNTR_SDMA: c_uint = 0x10;

pub const CNTR_MODE_W: c_uint = 0x0;
pub const CNTR_MODE_R: c_uint = 0x1;
// VLs Supported/Operational
pub const HFI1_MIN_VLS_SUPPORTED: c_int = 1;
pub const HFI1_MAX_VLS_SUPPORTED: c_int = 8;
pub const HFI1_GUIDS_PER_PORT: c_int = 5;
pub const HFI1_PORT_GUID_INDEX: c_int = 0;
pub const MAX_NAME_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_msix_entry {
    pub type: irq_type,
    pub irq: c_int,
    pub arg: *mut c_void,
    pub mask: cpumask_t,
    pub notify: irq_affinity_notify,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_msix_info {
// lock to synchronize in_use_msix access
    pub msix_lock: spinlock_t,
    pub CCE_NUM_MSIX_VECTORS): DECLARE_BITMAP(in_use_msix,,
    pub msix_entries: *mut hfi1_msix_entry,
    pub max_requested: u16,
}

// per-SL CCA information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cca_timer {
    pub hrtimer: hrtimer,
    pub /: *mut *mut *mut hfi1_pportdata ppd; / read-only,
    pub /: *mut *mut int sl; / read-only,
    pub /: *mut *mut u16 ccti; / read/write - current value of CCTI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_down_reason {
//
// SMA-facing value.  Should be set from .latest when
// HLS_UP_* -> HLS_DN_* transition actually occurs.
//
    pub sma: u8,
    pub latest: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vl_arb_cache {
// protect vl arb cache
    pub lock: spinlock_t,
    pub table: [ib_vl_weight_elem; VL_ARB_TABLE_SIZE],
}

//
// The structure below encapsulates data relevant to a physical IB Port.
// Current chips support only one such port, but the separation
// clarifies things a bit. Note that to conform to IB conventions,
// port-numbers are one-based. The first or only port is port1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_pportdata {
    pub ibport_data: hfi1_ibport,
    pub dd: *mut hfi1_devdata,
// PHY support
    pub qsfp_info: qsfp_data,
// Values for SI tuning of SerDes
    pub port_type: u32,
    pub tx_preset_eq: u32,
    pub tx_preset_noeq: u32,
    pub rx_preset: u32,
    pub local_atten: u8,
    pub remote_atten: u8,
    pub default_atten: u8,
    pub max_power_class: u8,
// did we read platform config from scratch registers?
    pub config_from_scratch: bool,
// GUIDs for this interface, in host order, guids[0] is a port guid
    pub guids: [u64; HFI1_GUIDS_PER_PORT],
// GUID for peer interface, in host order
    pub neighbor_guid: u64,
// up or down physical link state
    pub linkup: u32,
//
// this address is mapped read-only into user processes so they can
// get status cheaply, whenever they want.  One qword of status per port
//
    pub statusp: *mut u64,
// SendDMA related entries
    pub hfi1_wq: *mut workqueue_struct,
    pub link_wq: *mut workqueue_struct,
// move out of interrupt context
    pub link_vc_work: work_struct,
    pub link_up_work: work_struct,
    pub link_down_work: work_struct,
    pub sma_message_work: work_struct,
    pub freeze_work: work_struct,
    pub link_downgrade_work: work_struct,
    pub link_bounce_work: work_struct,
    pub start_link_work: delayed_work,
// host link state variables
    pub hls_lock: mutex,
    pub host_link_state: u32,
// these are the "32 bit" regs
    pub /: *mut *mut u32 ibmtu; / The MTU programmed for this unit,
//
// Current max size IB packet (in bytes) including IB headers, that
// we can send. Changes when ibmtu changes.
//
    pub ibmaxlen: u32,
    pub /: *mut *mut u32 current_egress_rate; / units [10^6 bits/sec],
// LID programmed for this instance
    pub lid: u32,
// list of pkeys programmed; 0 if not set
    pub pkeys: [u16; MAX_PKEY_VALUES],
    pub link_width_supported: u16,
    pub link_width_downgrade_supported: u16,
    pub link_speed_supported: u16,
    pub link_width_enabled: u16,
    pub link_width_downgrade_enabled: u16,
    pub link_speed_enabled: u16,
    pub link_width_active: u16,
    pub link_width_downgrade_tx_active: u16,
    pub link_width_downgrade_rx_active: u16,
    pub link_speed_active: u16,
    pub vls_supported: u8,
    pub vls_operational: u8,
    pub actual_vls_operational: u8,
// LID mask control
    pub lmc: u8,
// Rx Polarity inversion (compensate for ~tx on partner)
    pub rx_pol_inv: u8,
    pub /: *mut *mut u8 hw_pidx; / physical port index,
    pub /: *mut *mut u32 port; / IB port number and index into dd->pports - 1,
// type of neighbor node
    pub neighbor_type: u8,
    pub neighbor_normal: u8,
    pub /: *mut *mut u8 neighbor_fm_security; / 1 if firmware checking is disabled,
    pub neighbor_port_number: u8,
    pub is_sm_config_started: u8,
    pub offline_disabled_reason: u8,
    pub is_active_optimize_enabled: u8,
    pub /: *mut *mut u8 driver_link_ready; / driver ready for active link,
    pub /: *mut *mut u8 link_enabled; / link enabled?,
    pub linkinit_reason: u8,
    pub /: *mut *mut u8 local_tx_rate; / rate given to 8051 firmware,
    pub qsfp_retry_count: u8,
// placeholders for IB MAD packet settings
    pub overrun_threshold: u8,
    pub phy_error_threshold: u8,
    pub is_link_down_queued: c_uint,
// Used to override LED behavior for things like maintenance beaconing
//
// Alternates per phase of blink
// [0] holds LED off duration, [1] holds LED on duration
//
    pub led_override_vals: [c_ulong; 2],
    pub /: *mut *mut u8 led_override_phase; / LSB picks from vals[],
    pub led_override_timer_active: core::sync::atomic::AtomicI32,
// Used to flash LEDs in override mode
    pub led_override_timer: timer_list,
    pub sm_trap_qp: u32,
    pub sa_qp: u32,
//
// cca_timer_lock protects access to the per-SL cca_timer
// structures (specifically the ccti member).
//
    pub ____cacheline_aligned_in_smp: spinlock_t cca_timer_lock,
    pub cca_timer: [cca_timer; OPA_MAX_SLS],
// List of congestion control table entries
    pub ccti_entries: [ib_cc_table_entry_shadow; CC_TABLE_SHADOW_MAX],
// congestion entries, each entry corresponding to a SL
//
// cc_state_lock protects (write) access to the per-port
// struct cc_state.
//
    pub ____cacheline_aligned_in_smp: spinlock_t cc_state_lock,
    pub cc_state: *mut cc_state __rcu,
// Total number of congestion control table entries
    pub total_cct_entry: u16,
// Bit map identifying service level
    pub cc_sl_control_map: u32,
// CA's max number of 64 entry units in the congestion control table
    pub cc_max_table_entries: u8,
//
// begin congestion log related entries
// cc_log_lock protects all congestion log related data
//
    pub ____cacheline_aligned_in_smp: spinlock_t cc_log_lock,
    pub 8]: u8 threshold_cong_event_map[OPA_MAX_SLS /,
    pub threshold_event_counter: u16,
    pub cc_events: [opa_hfi1_cong_log_event_internal; OPA_CONG_LOG_ELEMS],
    pub /: *mut *mut int cc_log_idx; / index for logging events,
    pub /: *mut *mut int cc_mad_idx; / index for reporting events,
// end congestion log related entries
    pub vl_arb_cache: [vl_arb_cache; MAX_PRIO_TABLE],
// port relative counter buffer
    pub cntrs: *mut u64,
// port relative synthetic counter buffer
    pub scntrs: *mut u64,
// port_xmit_discards are synthesized from different egress errors
    pub port_xmit_discards: u64,
    pub port_xmit_discards_vl: [u64; C_VL_COUNT],
    pub port_xmit_constraint_errors: u64,
    pub port_rcv_constraint_errors: u64,
// count of 'link_err' interrupts from DC
    pub link_downed: u64,
// number of times link retrained successfully
    pub link_up: u64,
// number of times a link unknown frame was reported
    pub unknown_frame_count: u64,
// port_ltp_crc_mode is returned in 'portinfo' MADs
    pub port_ltp_crc_mode: u16,
// port_crc_mode_enabled is the crc we support
    pub port_crc_mode_enabled: u8,
// mgmt_allowed is also returned in 'portinfo' MADs
    pub mgmt_allowed: u8,
    pub /: *mut *mut u8 part_enforce; / partition enforcement flags,
    pub local_link_down_reason: link_down_reason,
    pub neigh_link_down_reason: link_down_reason,
// Value to be sent to link peer on LinkDown .
    pub remote_link_down_reason: u8,
// Error events that will cause a port bounce.
    pub port_error_action: u32,
    pub linkstate_active_work: work_struct,
// Does this port need to prescan for FECNs
    pub cc_prescan: bool,
//
// Sample sendWaitCnt & sendWaitVlCnt during link transition
// and counter request.
//
    pub 1]: u64 port_vl_xmit_wait_last[C_VL_COUNT +,
    pub prev_link_width: u16,
    pub 1]: u64 vl_xmit_flit_cnt[C_VL_COUNT +,
}

extern "C" {
    pub fn void(packet: *mut *mut opcode_handler)(struct hfi1_packet) -> typedef;
}
// return values for the RHF receive functions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcv_array_data {
    pub ngroups: u16,
    pub nctxt_extra: u16,
    pub group_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_vl_data {
    pub mtu: u16,
    pub sc: *mut send_context,
}

// 16 to directly index
pub const PER_VL_SEND_CONTEXTS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct err_info_rcvport {
    pub status_and_code: u8,
    pub packet_flit1: u64,
    pub packet_flit2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct err_info_constraint {
    pub status: u8,
    pub pkey: u16,
    pub slid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_temp {
    pub /: *mut *mut unsigned int curr; / current temperature,
    pub /: *mut *mut unsigned int lo_lim; / low temperature limit,
    pub /: *mut *mut unsigned int hi_lim; / high temperature limit,
    pub /: *mut *mut unsigned int crit_lim; / critical temperature limit,
    pub /: *mut *mut u8 triggers; / temperature triggers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_i2c_bus {
    pub /: *mut *mut *mut hfi1_devdata controlling_dd; / current controlling device,
    pub /: *mut *mut i2c_adapter adapter; / bus details,
    pub /: *mut *mut i2c_algo_bit_data algo; / bus algorithm details,
    pub /: *mut *mut int num; / bus number, 0 or 1,
}

// common data between shared ASIC HFIs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_asic_data {
    pub /: *mut *mut *mut hfi1_devdata dds[2]; / back pointers,
    pub asic_resource_mutex: mutex,
    pub i2c_bus0: *mut hfi1_i2c_bus,
    pub i2c_bus1: *mut hfi1_i2c_bus,
}

// sizes for both the QP and RSM map tables
pub const NUM_MAP_ENTRIES: c_int = 256;
pub const NUM_MAP_REGS: c_int = 32;
// device data struct now contains only "general per-device" info.
// fields related to a physical IB port are in a hfi1_pportdata struct.
//

extern "C" {
    pub fn int(: *mut *mut send_routine)(struct rvt_qp, : *mut hfi1_pkt_state, _arg: u64) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_devdata {
    pub /: *mut *mut hfi1_ibdev verbs_dev; / must be first,
// pointers to related structs for this device
// pci access data structure
    pub pcidev: *mut pci_dev,
    pub user_cdev: cdev,
    pub diag_cdev: cdev,
    pub ui_cdev: cdev,
    pub user_device: *mut device,
    pub diag_device: *mut device,
    pub ui_device: *mut device,
// first mapping up to RcvArray
    pub kregbase1: *mut u8 __iomem,
    pub physaddr: resource_size_t,
// second uncached mapping from RcvArray to pio send buffers
    pub kregbase2: *mut u8 __iomem,
// for detecting offset above kregbase2 address
    pub base2_start: u32,
// Per VL data. Enough for all VLs but not all elements are set/used.
    pub vld: [per_vl_data; PER_VL_SEND_CONTEXTS],
// send context data
    pub send_contexts: *mut send_context_info,
// map hardware send contexts to software index
    pub hw_to_sw: *mut u8,
// spinlock for allocating and releasing send context resources
    pub sc_lock: spinlock_t,
// lock for pio_map
    pub pio_map_lock: spinlock_t,
// Send Context initialization lock.
    pub sc_init_lock: spinlock_t,
// lock for sdma_map
    pub sde_map_lock: spinlock_t,
// array of kernel send contexts
    pub kernel_send_context: *mut send_context,
// array of vl maps
    pub pio_map: *mut pio_vl_map __rcu,
// default flags to last descriptor
    pub default_desc1: u64,
// fields common to all SDMA engines
    pub /: *mut *mut *mut volatile __le64 sdma_heads_dma; / DMA'ed by chip,
    pub sdma_heads_phys: dma_addr_t,
    pub /: *mut *mut *mut void sdma_pad_dma; / DMA'ed by chip,
    pub sdma_pad_phys: dma_addr_t,
// for deallocation
    pub sdma_heads_size: usize,
// num used
    pub num_sdma: u32,
// array of engines sized by num_sdma
    pub per_sdma: *mut sdma_engine,
// array of vl maps
    pub sdma_map: *mut sdma_vl_map __rcu,
// SPC freeze waitqueue and variable
    pub sdma_unfreeze_wq: wait_queue_head_t,
    pub sdma_unfreeze_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut u32 lcb_access_count; / count of LCB users,
// common data between shared ASIC HFIs in this OS
    pub asic_data: *mut hfi1_asic_data,
// mem-mapped pointer to base of PIO buffers
    pub piobase: *mut void __iomem,
//
// write-combining mem-mapped pointer to base of RcvArray
// memory.
//
    pub rcvarray_wc: *mut void __iomem,
//
// credit return base - a per-NUMA range of DMA address that
// the chip will use to update the per-context free counter
//
    pub cr_base: *mut credit_return_base,
// send context numbers and sizes for each type
    pub sc_sizes: [sc_config_sizes; SC_MAX],
    pub /: *mut *mut *mut char boardname; / human readable board info,
    pub ctx0_seq_drop: u64,
// reset value
    pub z_int_counter: u64,
    pub z_rcv_limit: u64,
    pub z_send_schedule: u64,
    pub send_schedule: *mut u64 __percpu,
// number of reserved contexts for netdev usage
    pub num_netdev_contexts: u16,
// number of receive contexts in use by the driver
    pub num_rcv_contexts: u32,
// number of pio send contexts in use by the driver
    pub num_send_contexts: u32,
//
// number of ctxts available for PSM open
//
    pub freectxts: u32,
// total number of available user/PSM contexts
    pub num_user_contexts: u32,
// base receive interrupt timeout, in CSR units
    pub rcv_intr_timeout_csr: u32,
    pub /: *mut *mut spinlock_t sendctrl_lock; / protect changes to SendCtrl,
    pub /: *mut *mut spinlock_t rcvctrl_lock; / protect changes to RcvCtrl,
    pub /: *mut *mut spinlock_t uctxt_lock; / protect rcd changes,
    pub /: *mut *mut mutex dc8051_lock; / exclusive access to 8051,
    pub update_cntr_wq: *mut workqueue_struct,
    pub update_cntr_work: work_struct,
// exclusive access to 8051 memory
    pub dc8051_memlock: spinlock_t,
    pub /: *mut *mut int dc8051_timed_out; / remember if the 8051 timed out,
//
// A page that will hold event notification bitmaps for all
// contexts. This page will be mapped into all processes.
//
    pub events: *mut c_ulong,
//
// per unit status, see also portdata statusp
// mapped read-only into user processes so they can get unit and
// IB link status cheaply
//
    pub status: *mut hfi1_status,
// revision register shadow
    pub revision: u64,
// Base GUID for device (network order)
    pub base_guid: u64,
// both sides of the PCIe link are gen3 capable
    pub link_gen3_capable: u8,
    pub dc_shutdown: u8,
// localbus width (1, 2,4,8,16,32) from config space
    pub lbus_width: u32,
// localbus speed in MHz
    pub lbus_speed: u32,
    pub /: *mut *mut int unit; / unit # of this chip,
    pub /: *mut *mut int node; / home node of this chip,
// save these PCI fields to restore after a reset
    pub pcibar0: u32,
    pub pcibar1: u32,
    pub pci_rom: u32,
    pub pci_command: u16,
    pub pcie_devctl: u16,
    pub pcie_lnkctl: u16,
    pub pcie_devctl2: u16,
    pub pci_msix0: u32,
    pub pci_tph2: u32,
//
// ASCII serial number, from flash, large enough for original
// all digit strings, and longer serial number format
//
    pub serial: [u8; SERIAL_MAX],
// human readable board version
    pub boardversion: [u8; BOARD_VERS_MAX],
    pub /: *mut *mut u8 lbus_info[32]; / human readable localbus info,
// chip major rev, from CceRevision
    pub majrev: u8,
// chip minor rev, from CceRevision
    pub minrev: u8,
// hardware ID
    pub hfi1_id: u8,
// implementation code
    pub icode: u8,
// vAU of this device
    pub vau: u8,
// vCU of this device
    pub vcu: u8,
// link credits of this device
    pub link_credits: u16,
// initial vl15 credits to use
    pub vl15_init: u16,
//
// Cached value for vl15buf, read during verify cap interrupt. VL15
// credits are to be kept at 0 and set when handling the link-up
// interrupt. This removes the possibility of receiving VL15 MAD
// packets before this HFI is ready.
//
    pub vl15buf_cached: u16,
// Misc small ints
    pub n_krcv_queues: u8,
    pub qos_shift: u8,
    pub /: *mut *mut u16 irev; / implementation revision,
    pub /: *mut *mut u32 dc8051_ver; / 8051 firmware version,
    pub /: *mut *mut spinlock_t hfi1_diag_trans_lock; / protect diag observer ops,
    pub platform_config: platform_config,
    pub pcfg_cache: platform_config_cache,
    pub diag_client: *mut diag_client,
// general interrupt: mask of handled interrupts
    pub gi_mask: [u64; CCE_NUM_INT_CSRS],
    pub rcv_entries: rcv_array_data,
// cycle length of PS* counters in HW (in picoseconds)
    pub psxmitwait_check_rate: u16,
//
// 64 bit synthetic counters
//
    pub synth_stats_timer: timer_list,
// MSI-X information
    pub msix_info: hfi1_msix_info,
//
// device counters
//
    pub cntrnames: *mut c_char,
    pub cntrnameslen: usize,
    pub ndevcntrs: usize,
    pub cntrs: *mut u64,
    pub scntrs: *mut u64,
//
// remembered values for synthetic counters
//
    pub last_tx: u64,
    pub last_rx: u64,
//
// per-port counters
//
    pub nportcntrs: usize,
    pub portcntrnames: *mut c_char,
    pub portcntrnameslen: usize,
    pub err_info_rcvport: err_info_rcvport,
    pub err_info_rcv_constraint: err_info_constraint,
    pub err_info_xmit_constraint: err_info_constraint,
    pub drop_packet: core::sync::atomic::AtomicI32,
    pub do_drop: bool,
    pub err_info_uncorrectable: u8,
    pub err_info_fmconfig: u8,
//
// Software counters for the status bits defined by the
// associated error status registers
//
    pub cce_err_status_cnt: [u64; NUM_CCE_ERR_STATUS_COUNTERS],
    pub rcv_err_status_cnt: [u64; NUM_RCV_ERR_STATUS_COUNTERS],
    pub misc_err_status_cnt: [u64; NUM_MISC_ERR_STATUS_COUNTERS],
    pub send_pio_err_status_cnt: [u64; NUM_SEND_PIO_ERR_STATUS_COUNTERS],
    pub send_dma_err_status_cnt: [u64; NUM_SEND_DMA_ERR_STATUS_COUNTERS],
    pub send_egress_err_status_cnt: [u64; NUM_SEND_EGRESS_ERR_STATUS_COUNTERS],
    pub send_err_status_cnt: [u64; NUM_SEND_ERR_STATUS_COUNTERS],
// Software counter that spans all contexts
    pub sw_ctxt_err_status_cnt: [u64; NUM_SEND_CTXT_ERR_STATUS_COUNTERS],
// Software counter that spans all DMA engines
// Software counter that aggregates all cce_err_status errors
    pub sw_cce_err_status_aggregate: u64,
// Software counter that aggregates all bypass packet rcv errors
    pub sw_rcv_bypass_packet_errors: u64,
// Save the enabled LCB error bits
    pub lcb_err_en: u64,
    pub comp_vect: *mut cpu_mask_set,
    pub comp_vect_mappings: *mut c_int,
    pub comp_vect_possible_cpus: u32,
//
// Capability to have different send engines simply by changing a
// pointer value.
//
    pub ____cacheline_aligned_in_smp: send_routine process_pio_send,
    pub process_dma_send: send_routine,
    pub count): *const *const u64 pbc, void from, size_t,
// hfi1_pportdata, points to array of (physical) port-specific
// data structs, indexed by pidx (0..n-1)
//
    pub pport: *mut hfi1_pportdata,
// receive context data
    pub rcd: *mut hfi1_ctxtdata,
    pub int_counter: *mut u64 __percpu,
// verbs tx opcode stats
    pub tx_opstats: *mut hfi1_opcode_stats_perctx __percpu,
// device (not port) flags, basically device capabilities
    pub flags: u16,
// Number of physical ports available
    pub num_pports: u8,
    pub first_dyn_alloc_ctxt: u8,
// adding a new field here would make it part of this cacheline
// seqlock for sc2vl
    pub ____cacheline_aligned_in_smp: seqlock_t sc2vl_lock,
    pub sc2vl: [u64; 4],
    pub rcv_limit: *mut u64 __percpu,
// adding a new field here would make it part of this cacheline
// OUI comes from the HW. Used everywhere as 3 separate bytes.
    pub oui1: u8,
    pub oui2: u8,
    pub oui3: u8,
// Timer and counter used to detect RcvBufOvflCnt changes
    pub rcverr_timer: timer_list,
    pub event_queue: wait_queue_head_t,
// receive context tail dummy address
    pub rcvhdrtail_dummy_kvaddr: *mut __le64,
    pub rcvhdrtail_dummy_dma: dma_addr_t,
    pub rcv_ovfl_cnt: u32,
// Serialize ASPM enable/disable between multiple verbs contexts
    pub aspm_lock: spinlock_t,
// Number of verbs contexts which have disabled ASPM
    pub aspm_disabled_cnt: core::sync::atomic::AtomicI32,
// Keeps track of user space clients
    pub user_refcount: refcount_t,
// Used to wait for outstanding user space clients before dev removal
    pub user_comp: completion,
    pub /: *mut *mut bool eprom_available; / true if EPROM is available for this device,
    pub /: *mut *mut bool aspm_supported; / Does HW support ASPM,
    pub /: *mut *mut bool aspm_enabled; / ASPM state: enabled/disabled,
    pub sdma_rht: *mut rhashtable,
// Lock to protect IRQ SRC register access
    pub irq_src_lock: spinlock_t,
    pub netdev_rx: *mut hfi1_netdev_rx,
    pub affinity_entry: *mut hfi1_affinity_node,
// Keeps track of IPoIB RSM rule users
    pub ipoib_rsm_usr_num: core::sync::atomic::AtomicI32,
}

// 8051 firmware version helper

// f_put_tid types
pub const PT_EXPECTED: c_int = 0;
pub const PT_EAGER: c_int = 1;
pub const PT_INVALID_FLUSH: c_int = 2;
pub const PT_INVALID: c_int = 3;
// Private data for file operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_filedata {
    pub pq_srcu: srcu_struct,
    pub dd: *mut hfi1_devdata,
    pub uctxt: *mut hfi1_ctxtdata,
    pub cq: *mut hfi1_user_sdma_comp_q,
// update side lock for SRCU
    pub pq_rcu_lock: spinlock_t,
    pub pq: *mut hfi1_user_sdma_pkt_q __rcu,
    pub subctxt: u16,
// for cpu affinity; -1 if none
    pub rec_cpu_num: c_int,
    pub tid_n_pinned: u32,
    pub use_mn: bool,
    pub entry_to_rb: *mut tid_rb_node,
    pub /: *mut *mut spinlock_t tid_lock; / protect tid_[limit,used] counters,
    pub tid_limit: u32,
    pub tid_used: u32,
    pub invalid_tids: *mut u32,
    pub invalid_tid_idx: u32,
// protect invalid_tids array and invalid_tid_idx
    pub invalid_lock: spinlock_t,
}

extern "C" {
    pub fn hfi1_init(dd: *mut hfi1_devdata, reinit: c_int) -> c_int;
}
extern "C" {
    pub fn hfi1_count_active_units() -> c_int;
}
extern "C" {
    pub fn hfi1_diag_add(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_diag_remove(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn handle_linkup_change(dd: *mut hfi1_devdata, linkup: u32);
}
extern "C" {
    pub fn handle_user_interrupt(rcd: *mut hfi1_ctxtdata);
}
extern "C" {
    pub fn hfi1_create_rcvhdrq(dd: *mut hfi1_devdata, rcd: *mut hfi1_ctxtdata) -> c_int;
}
extern "C" {
    pub fn hfi1_setup_eagerbufs(rcd: *mut hfi1_ctxtdata) -> c_int;
}
extern "C" {
    pub fn hfi1_create_kctxts(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_free_ctxt(rcd: *mut hfi1_ctxtdata);
}
extern "C" {
    pub fn hfi1_free_ctxtdata(dd: *mut hfi1_devdata, rcd: *mut hfi1_ctxtdata);
}
extern "C" {
    pub fn hfi1_rcd_put(rcd: *mut hfi1_ctxtdata) -> c_int;
}
extern "C" {
    pub fn hfi1_rcd_get(rcd: *mut hfi1_ctxtdata) -> c_int;
}
extern "C" {
    pub fn handle_receive_interrupt(rcd: *mut hfi1_ctxtdata, thread: c_int) -> c_int;
}
extern "C" {
    pub fn handle_receive_interrupt_nodma_rtail(rcd: *mut hfi1_ctxtdata, thread: c_int) -> c_int;
}
extern "C" {
    pub fn handle_receive_interrupt_dma_rtail(rcd: *mut hfi1_ctxtdata, thread: c_int) -> c_int;
}
extern "C" {
    pub fn handle_receive_interrupt_napi_fp(rcd: *mut hfi1_ctxtdata, budget: c_int) -> c_int;
}
extern "C" {
    pub fn handle_receive_interrupt_napi_sp(rcd: *mut hfi1_ctxtdata, budget: c_int) -> c_int;
}
extern "C" {
    pub fn set_all_slowpath(dd: *mut hfi1_devdata);
}
// receive packet handler dispositions
pub const RCV_PKT_OK: c_uint = 0x0 /* keep going */;
pub const RCV_PKT_LIMIT: c_uint = 0x1 /* stop, hit limit, start thread */;
pub const RCV_PKT_DONE: c_uint = 0x2 /* stop, no more packets detected */;
//
// hfi1_rcd_head - add accessor for rcd head
// @rcd: the context
//
// hfi1_set_rcd_head - add accessor for rcd head
// @rcd: the context
// @head: the new head
//
// calculate the current RHF address
// return DMA_RTAIL configuration
//
// hfi1_seq_incr_wrap - wrapping increment for sequence
// @seq: the current sequence number
//
// Returns: the incremented seq
//
// hfi1_seq_cnt - return seq_cnt member
// @rcd: the receive context
//
// Return seq_cnt member
//
// hfi1_set_seq_cnt - return seq_cnt member
// @rcd: the receive context
//
// Return seq_cnt member
//
// last_rcv_seq - is last
// @rcd: the receive context
// @seq: sequence
//
// return true if last packet
//
// rcd_seq_incr - increment context sequence number
// @rcd: the receive context
// @seq: the current sequence number
//
// Returns: true if the this was the last packet
//
extern "C" {
    pub fn last_rcv_seq(_arg: rcd, _arg: seq) -> return;
}
//
// get_hdrqentsize - return hdrq entry size
// @rcd: the receive context
//
// get_hdrq_cnt - return hdrq count
// @rcd: the receive context
//
// hfi1_is_slowpath - check if this context is slow path
// @rcd: the receive context
//
// hfi1_is_fastpath - check if this context is fast path
// @rcd: the receive context
//
// hfi1_set_fast - change to the fast handler
// @rcd: the receive context
//
extern "C" {
    pub fn hfi1_reset_device(_arg: c_int) -> c_int;
}
extern "C" {
    pub fn receive_interrupt_work(work: *mut work_struct);
}
// extract service channel from header and rhf
extern "C" {
    pub fn ib_get_sc(4: hdr) | ((!!(rhf_dc_info(rhf))) <<) -> return;
}
pub const HFI1_JKEY_WIDTH: c_int = 16;

pub const HFI1_ADMIN_JKEY_RANGE: c_int = 32;
//
// J_KEYs are split and allocated in the following groups:
// 0 - 31    - users with administrator privileges
// 32 - 63    - kernel protocols using KDETH packets
// 64 - 65535 - all other users using KDETH packets
//
// active_egress_rate
//
// returns the active egress rate in units of [10^6 bits/sec]
//
// assume IB_WIDTH_1X
//
// egress_cycles
//
// Returns the number of 'fabric clock cycles' to egress a packet
// of length 'len' bytes, at 'rate' Mbit/s. Since the fabric clock
// rate is (approximately) 805 MHz, the units of the returned value
// are (1/805 MHz).
//
// cycles is:
//
// (length) [bits] / (rate) [bits/sec]
// ---------------------------------------------------
// fabric_clock_period == 1 /(805 * 10^6) [cycles/sec]
//
extern "C" {
    pub fn set_link_ipg(ppd: *mut hfi1_pportdata);
}

pub const PACKET_EGRESS_TIMEOUT: c_int = 350;
// Pause at least 1us, to ensure chip returns all credits
//
// sc_to_vlt() - reverse lookup sc to vl
// @dd - devdata
// @sc5 - 5 bit sc
//
pub const PKEY_MEMBER_MASK: c_uint = 0x8000;
pub const PKEY_LOW_15_MASK: c_uint = 0x7fff;
//
// ingress_pkey_matches_entry - return 1 if the pkey matches ent (ent
// being an entry from the ingress partition key table), return 0
// otherwise. Use the matching criteria for ingress partition keys
// specified in the OPAv1 spec., section 9.10.14.
//
// If pkey[15] is clear (limited partition member),
// is bit 15 in the corresponding table element
// clear (limited member)?
//
// ingress_pkey_table_search - search the entire pkey table for
// an entry which matches 'pkey'. return 0 if a match is found,
// and 1 otherwise.
//
// ingress_pkey_table_fail - record a failure of ingress pkey validation,
// i.e., increment port_rcv_constraint_errors for the port, and record
// the 'error info' for this failure.
//
// ingress_pkey_check - Return 0 if the ingress pkey is valid, return 1
// otherwise. Use the criteria in the OPAv1 spec, section 9.10.14. idx
// is a hint as to the best place in the partition key table to begin
// searching. This function should not be called on the data path because
// of performance reasons. On datapath pkey check is expected to be done
// by HW and rcv_pkey_check function should be called instead.
//
// If SC15, pkey[0:14] must be 0x7fff
// Is the pkey = 0x0, or 0x8000?
// The most likely matching pkey has index 'idx'
// no match - try the whole table
//
// rcv_pkey_check - Return 0 if the ingress pkey is valid, return 1
// otherwise. It only ensures pkey is vlid for QP0. This function
// should be called on the data path instead of ingress_pkey_check
// as on data path, pkey check is done by HW (except for QP0).
//
// If SC15, pkey[0:14] must be 0x7fff
// MTU handling
// MTU enumeration, 256-4k match IB
pub const OPA_MTU_0: c_int = 0;
pub const OPA_MTU_256: c_int = 1;
pub const OPA_MTU_512: c_int = 2;
pub const OPA_MTU_1024: c_int = 3;
pub const OPA_MTU_2048: c_int = 4;
pub const OPA_MTU_4096: c_int = 5;
extern "C" {
    pub fn lrh_max_header_bytes(dd: *mut hfi1_devdata) -> u32;
}
extern "C" {
    pub fn mtu_to_enum(mtu: u32, default_if_bad: c_int) -> c_int;
}
extern "C" {
    pub fn enum_to_mtu(mtu: c_int) -> u16;
}
extern "C" {
    pub fn set_mtu(ppd: *mut hfi1_pportdata) -> c_int;
}
extern "C" {
    pub fn hfi1_set_lid(ppd: *mut hfi1_pportdata, lid: u32, lmc: u8) -> c_int;
}
extern "C" {
    pub fn hfi1_disable_after_error(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_set_uevent_bits(ppd: *mut hfi1_pportdata, evtbit: c_int) -> c_int;
}
extern "C" {
    pub fn hfi1_rcvbuf_validate(size: u32, type: u8, encode: *mut u16) -> c_int;
}
extern "C" {
    pub fn fm_get_table(ppd: *mut hfi1_pportdata, which: c_int, t: *mut c_void) -> c_int;
}
extern "C" {
    pub fn fm_set_table(ppd: *mut hfi1_pportdata, which: c_int, t: *mut c_void) -> c_int;
}
extern "C" {
    pub fn set_up_vau(dd: *mut hfi1_devdata, vau: u8);
}
extern "C" {
    pub fn set_up_vl15(dd: *mut hfi1_devdata, vl15buf: u16);
}
extern "C" {
    pub fn reset_link_credits(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn assign_remote_cm_au_table(dd: *mut hfi1_devdata, vcu: u8);
}
extern "C" {
    pub fn set_buffer_control(ppd: *mut hfi1_pportdata, bc: *mut buffer_control) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: dev, hfi1_devdata: struct, _arg: verbs_dev) -> return;
}
extern "C" {
    pub fn dd_from_dev(_arg: to_idev(ibdev)) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibp, hfi1_pportdata: struct, _arg: ibport_data) -> return;
}
extern "C" {
    pub fn container_of(_arg: rdi, hfi1_ibdev: struct, _arg: rdi) -> return;
}
//
// hfi1_may_ecn - Check whether FECN or BECN processing should be done
// @pkt: the packet to be evaluated
//
// Check whether the FECN or BECN bits in the packet's header are
// enabled, depending on packet type.
//
// This function only checks for FECN and BECN bits. Additional checks
// are done in the slowpath (hfi1_process_ecn_slowpath()) in order to
// ensure correct handling.
//
extern "C" {
    pub fn hfi1_process_ecn_slowpath(_arg: qp, _arg: pkt, _arg: false) -> return;
}
//
// Return the indexed PKEY from the port PKEY table.
//
// Return the indexed GUID from the port GUIDs table.
//
extern "C" {
    pub fn cpu_to_be64(_arg: ppd->guids[index]) -> return;
}
//
// Called by readers of cc_state only, must call under rcu_read_lock().
//
extern "C" {
    pub fn rcu_dereference(_arg: ppd->cc_state) -> return;
}
//
// Called by writers of cc_state only,  must call under cc_state_lock.
//
// values for dd->flags (_device_ related flags)
//
pub const HFI1_INITTED: c_uint = 0x1    /* chip and driver up and initted */;
pub const HFI1_PRESENT: c_uint = 0x2    /* chip accesses can be done */;
pub const HFI1_FROZEN: c_uint = 0x4    /* chip in SPC freeze */;
pub const HFI1_HAS_SDMA_TIMEOUT: c_uint = 0x8;
pub const HFI1_HAS_SEND_DMA: c_uint = 0x10   /* Supports Send DMA */;
pub const HFI1_FORCED_FREEZE: c_uint = 0x80   /* driver forced freeze mode */;
pub const HFI1_SHUTDOWN: c_uint = 0x100  /* device is shutting down */;
// IB dword length mask in PBC (lower 11 bits); same for all chips

// ctxt_flag bit offsets
// base context has not finished initializing
pub const HFI1_CTXT_BASE_UNINIT: c_int = 1;
// base context initaliation failed
pub const HFI1_CTXT_BASE_FAILED: c_int = 2;
// waiting for a packet to arrive
pub const HFI1_CTXT_WAITING_RCV: c_int = 3;
// waiting for an urgent packet to arrive
pub const HFI1_CTXT_WAITING_URG: c_int = 4;
extern "C" {
    pub fn hfi1_init_dd(dd: *mut hfi1_devdata) -> c_int;
}
// LED beaconing functions
extern "C" {
    pub fn shutdown_led_override(ppd: *mut hfi1_pportdata);
}

//
// The number of words for the KDETH protocol field.  If this is
// larger then the actual field used, then part of the payload
// will be in the header.
//
// Optimally, we want this sized so that a typical case will
// use full cache lines.  The typical local KDETH header would
// be:
//
// Bytes	Field
// 8	LRH
// 12	BHT
// ??	KDETH
// 8	RHF
// ---
// 28 + KDETH
//
// For a 64-byte cache line, KDETH would need to be 36 bytes or 9 DWORDS
//
pub const DEFAULT_RCVHDRSIZE: c_int = 9;
//
// Maximal header byte count:
//
// Bytes	Field
// 8	LRH
// 40	GRH (optional)
// 12	BTH
// ??	KDETH
// 8	RHF
// ---
// 68 + KDETH
//
// We also want to maintain a cache line alignment to assist DMA'ing
// of the header bytes.  Round up to a good size.
//
pub const DEFAULT_RCVHDR_ENTSIZE: c_int = 32;
//
// hfi1_rcvhdrtail_kvaddr - return tail kvaddr
// @rcd - the receive context
//
// kv = 0ULL;
//
// volatile because it's a DMA target from the chip, routine is
// inlined, and don't want register caching or reordering.
//
extern "C" {
    pub fn hfi1_rcd_head(get_rcvhdrtail(rcd: rcd) !=) -> return;
}
//
// sysfs interface.
//
extern "C" {
    pub fn hfi1_device_create(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_device_remove(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_verbs_register_sysfs(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_verbs_unregister_sysfs(dd: *mut hfi1_devdata);
}
// Hook for sysfs read of QSFP
extern "C" {
    pub fn qsfp_dump(ppd: *mut hfi1_pportdata, buf: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn hfi1_pcie_init(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn hfi1_pcie_cleanup(pdev: *mut pci_dev);
}
extern "C" {
    pub fn hfi1_pcie_ddinit(dd: *mut hfi1_devdata, pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn hfi1_pcie_ddcleanup(: *mut hfi1_devdata);
}
extern "C" {
    pub fn pcie_speeds(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn restore_pci_variables(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn save_pci_variables(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn do_pcie_gen3_transition(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn tune_pcie_caps(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn parse_platform_config(dd: *mut hfi1_devdata) -> c_int;
}
//
// Flush write combining store buffers (if present) and perform a write
// barrier.
//
extern "C" {
    pub fn volatile("memory": "sfence" : : :) -> asm;
}
extern "C" {
    pub fn handle_eflags(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn seqfile_dump_rcd(s: *mut seq_file, rcd: *mut hfi1_ctxtdata);
}
// global module parameter variables
// Number of seconds before our card status check...
pub const STATUS_TIMEOUT: c_int = 60;

pub const HFI1_USER_MINOR_BASE: c_int = 0;
pub const HFI1_TRACE_MINOR: c_int = 127;
pub const HFI1_NMINORS: c_int = 255;
pub const PCI_VENDOR_ID_INTEL: c_uint = 0x8086;
pub const PCI_DEVICE_ID_INTEL0: c_uint = 0x24f0;
pub const PCI_DEVICE_ID_INTEL1: c_uint = 0x24f1;

// No integrity checks if HFI1_CAP_NO_INTEGRITY is set

// turn on send-side job key checks if !A0
// No integrity checks if HFI1_CAP_NO_INTEGRITY is set
// turn on send-side job key checks if !A0

pub const USER_OPCODE_CHECK_VAL: c_uint = 0xC0;
pub const USER_OPCODE_CHECK_MASK: c_uint = 0xC0;
pub const OPCODE_CHECK_VAL_DISABLED: c_uint = 0x0;
pub const OPCODE_CHECK_MASK_DISABLED: c_uint = 0x0;
// Control LED state
// return the i2c resource given the target
// return the i2c chain chip resource that this HFI uses for QSFP
extern "C" {
    pub fn i2c_target(_arg: dd->hfi1_id) -> return;
}
// Is this device integrated or discrete?
//
// hfi1_need_drop - detect need for drop
// @dd: - the device
//
// In some cases, the first packet needs to be dropped.
//
// Return true is the current packet needs to be dropped and false otherwise.
//
extern "C" {
    pub fn hfi1_tempsense_rd(dd: *mut hfi1_devdata, temp: *mut hfi1_temp) -> c_int;
}

//
// Kernel clients may not have setup GRH information
// Set that here.
//
// hfi1_check_mcast- Check if the given lid is
// in the OPA multicast range.
//
// The LID might either reside in ah.dlid or might be
// in the GRH of the address handle as DGID if extended
// addresses are in use.
//

// Convert a lid to a specific lid space
// Return true if the given lid is the OPA 16B multicast range
// Modify ah_attr.dlid to be in the 32 bit LID space.
// This is how the address will be laid out:
// Assuming MCAST_NR to be 4,
// 32 bit permissive LID = 0xFFFFFFFF
// Multicast LID range = 0xFFFFFFFE to 0xF0000000
// Unicast LID range = 0xEFFFFFFF to 1
// Invalid LID = 0
//
// 9B if lid > 0xF0000000
// 16B if lid > 0xC000
//
// If there was an incoming 16B packet with permissive
// LIDs, OPA GIDs would have been programmed when those
// packets were received. A 16B packet will have to
// be sent in response to that packet. Return a 16B
// header type if that's the case.
//
// Return a 16B header type if either the destination
// or source lid is extended.
//
extern "C" {
    pub fn hfi1_get_packet_type(_arg: lid) -> return;
}
//
// Upper layers (like mad) may compare the dgid in the
// wc that is obtained here with the sgid_index in
// the wr. Since sgid_index in wr is always 0 for
// extended lids, set the dgid here to the default
// IB gid.
//
