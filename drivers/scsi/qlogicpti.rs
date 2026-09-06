//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qlogicpti.h
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
// qlogicpti.h: Performance Technologies QlogicISP sbus card defines.
//
// Copyright (C) 1996 David S. Miller (davem@caipfs.rutgers.edu)
//
// Qlogic/SBUS controller registers.
pub const SBUS_CFG1: c_uint = 0x006UL;
pub const SBUS_CTRL: c_uint = 0x008UL;
pub const SBUS_STAT: c_uint = 0x00aUL;
pub const SBUS_SEMAPHORE: c_uint = 0x00cUL;
pub const CMD_DMA_CTRL: c_uint = 0x022UL;
pub const DATA_DMA_CTRL: c_uint = 0x042UL;
pub const MBOX0: c_uint = 0x080UL;
pub const MBOX1: c_uint = 0x082UL;
pub const MBOX2: c_uint = 0x084UL;
pub const MBOX3: c_uint = 0x086UL;
pub const MBOX4: c_uint = 0x088UL;
pub const MBOX5: c_uint = 0x08aUL;
pub const CPU_CMD: c_uint = 0x214UL;
pub const CPU_ORIDE: c_uint = 0x224UL;
pub const CPU_PCTRL: c_uint = 0x272UL;
pub const CPU_PDIFF: c_uint = 0x276UL;
pub const RISC_PSR: c_uint = 0x420UL;
pub const RISC_MTREG: c_uint = 0x42EUL;
pub const HCCTRL: c_uint = 0x440UL;
// SCSI parameters for this driver.
pub const MAX_TARGETS: c_int = 16;
pub const MAX_LUNS: c_int = 8;
// With the qlogic interface, every queue slot can hold a SCSI
// command with up to 4 scatter/gather entries.  If we need more
// than 4 entries, continuation entries can be used that hold
// another 7 entries each.  Unlike for other drivers, this means
// that the maximum number of scatter/gather entries we can
// support at any given time is a function of the number of queue
// slots available.  That is, host->can_queue and host->sg_tablesize
// are dynamic and _not_ independent.  This all works fine because
// requests are queued serially and the scatter/gather limit is
// determined for each queue request anew.
//

// mailbox command complete status codes
pub const MBOX_COMMAND_COMPLETE: c_uint = 0x4000;
pub const INVALID_COMMAND: c_uint = 0x4001;
pub const HOST_INTERFACE_ERROR: c_uint = 0x4002;
pub const TEST_FAILED: c_uint = 0x4003;
pub const COMMAND_ERROR: c_uint = 0x4005;
pub const COMMAND_PARAM_ERROR: c_uint = 0x4006;
// async event status codes
pub const ASYNC_SCSI_BUS_RESET: c_uint = 0x8001;
pub const SYSTEM_ERROR: c_uint = 0x8002;
pub const REQUEST_TRANSFER_ERROR: c_uint = 0x8003;
pub const RESPONSE_TRANSFER_ERROR: c_uint = 0x8004;
pub const REQUEST_QUEUE_WAKEUP: c_uint = 0x8005;
pub const EXECUTION_TIMEOUT_RESET: c_uint = 0x8006;
// Am I fucking pedantic or what?
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entry_header {

    pub entry_cnt: u8,
    pub entry_type: u8,
    pub flags: u8,
    pub sys_def_1: u8,

    pub entry_type: u8,
    pub entry_cnt: u8,
    pub sys_def_1: u8,
    pub flags: u8,

}

// entry header type commands
pub const ENTRY_COMMAND: c_int = 1;
pub const ENTRY_CONTINUATION: c_int = 2;
pub const ENTRY_STATUS: c_int = 3;
pub const ENTRY_MARKER: c_int = 4;
pub const ENTRY_EXTENDED_COMMAND: c_int = 5;
// entry header flag definitions
pub const EFLAG_CONTINUATION: c_int = 1;
pub const EFLAG_BUSY: c_int = 2;
pub const EFLAG_BAD_HEADER: c_int = 4;
pub const EFLAG_BAD_PAYLOAD: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dataseg {
    pub d_base: u32,
    pub d_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Command_Entry {
    pub hdr: Entry_header,
    pub handle: u32,

    pub target_id: u8,
    pub target_lun: u8,

    pub target_lun: u8,
    pub target_id: u8,

    pub cdb_length: u16,
    pub control_flags: u16,
    pub rsvd: u16,
    pub time_out: u16,
    pub segment_cnt: u16,
    pub cdb: [u8; 12],
    pub dataseg: [dataseg; 4],
}

// command entry control flag definitions
pub const CFLAG_NODISC: c_uint = 0x01;
pub const CFLAG_HEAD_TAG: c_uint = 0x02;
pub const CFLAG_ORDERED_TAG: c_uint = 0x04;
pub const CFLAG_SIMPLE_TAG: c_uint = 0x08;
pub const CFLAG_TAR_RTN: c_uint = 0x10;
pub const CFLAG_READ: c_uint = 0x20;
pub const CFLAG_WRITE: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext_Command_Entry {
    pub hdr: Entry_header,
    pub handle: u32,

    pub target_id: u8,
    pub target_lun: u8,

    pub target_lun: u8,
    pub target_id: u8,

    pub cdb_length: u16,
    pub control_flags: u16,
    pub rsvd: u16,
    pub time_out: u16,
    pub segment_cnt: u16,
    pub cdb: [u8; 44],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Continuation_Entry {
    pub hdr: Entry_header,
    pub reserved: u32,
    pub dataseg: [dataseg; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Marker_Entry {
    pub hdr: Entry_header,
    pub reserved: u32,

    pub target_id: u8,
    pub target_lun: u8,

    pub target_lun: u8,
    pub target_id: u8,

    pub rsvd: u8,
    pub modifier: u8,

    pub modifier: u8,
    pub rsvd: u8,
    pub rsvds: [u8; 52],
}

// marker entry modifier definitions
pub const SYNC_DEVICE: c_int = 0;
pub const SYNC_TARGET: c_int = 1;
pub const SYNC_ALL: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Status_Entry {
    pub hdr: Entry_header,
    pub handle: u32,
    pub scsi_status: u16,
    pub completion_status: u16,
    pub state_flags: u16,
    pub status_flags: u16,
    pub time: u16,
    pub req_sense_len: u16,
    pub residual: u32,
    pub rsvd: [u8; 8],
    pub req_sense_data: [u8; 32],
}

// status entry completion status definitions
pub const CS_COMPLETE: c_uint = 0x0000;
pub const CS_INCOMPLETE: c_uint = 0x0001;
pub const CS_DMA_ERROR: c_uint = 0x0002;
pub const CS_TRANSPORT_ERROR: c_uint = 0x0003;
pub const CS_RESET_OCCURRED: c_uint = 0x0004;
pub const CS_ABORTED: c_uint = 0x0005;
pub const CS_TIMEOUT: c_uint = 0x0006;
pub const CS_DATA_OVERRUN: c_uint = 0x0007;
pub const CS_COMMAND_OVERRUN: c_uint = 0x0008;
pub const CS_STATUS_OVERRUN: c_uint = 0x0009;
pub const CS_BAD_MESSAGE: c_uint = 0x000a;
pub const CS_NO_MESSAGE_OUT: c_uint = 0x000b;
pub const CS_EXT_ID_FAILED: c_uint = 0x000c;
pub const CS_IDE_MSG_FAILED: c_uint = 0x000d;
pub const CS_ABORT_MSG_FAILED: c_uint = 0x000e;
pub const CS_REJECT_MSG_FAILED: c_uint = 0x000f;
pub const CS_NOP_MSG_FAILED: c_uint = 0x0010;
pub const CS_PARITY_ERROR_MSG_FAILED: c_uint = 0x0011;
pub const CS_DEVICE_RESET_MSG_FAILED: c_uint = 0x0012;
pub const CS_ID_MSG_FAILED: c_uint = 0x0013;
pub const CS_UNEXP_BUS_FREE: c_uint = 0x0014;
pub const CS_DATA_UNDERRUN: c_uint = 0x0015;
pub const CS_BUS_RESET: c_uint = 0x001c;
// status entry state flag definitions
pub const SF_GOT_BUS: c_uint = 0x0100;
pub const SF_GOT_TARGET: c_uint = 0x0200;
pub const SF_SENT_CDB: c_uint = 0x0400;
pub const SF_TRANSFERRED_DATA: c_uint = 0x0800;
pub const SF_GOT_STATUS: c_uint = 0x1000;
pub const SF_GOT_SENSE: c_uint = 0x2000;
// status entry status flag definitions
pub const STF_DISCONNECT: c_uint = 0x0001;
pub const STF_SYNCHRONOUS: c_uint = 0x0002;
pub const STF_PARITY_ERROR: c_uint = 0x0004;
pub const STF_BUS_RESET: c_uint = 0x0008;
pub const STF_DEVICE_RESET: c_uint = 0x0010;
pub const STF_ABORTED: c_uint = 0x0020;
pub const STF_TIMEOUT: c_uint = 0x0040;
pub const STF_NEGOTIATION: c_uint = 0x0080;
// mailbox commands
pub const MBOX_NO_OP: c_uint = 0x0000;
pub const MBOX_LOAD_RAM: c_uint = 0x0001;
pub const MBOX_EXEC_FIRMWARE: c_uint = 0x0002;
pub const MBOX_DUMP_RAM: c_uint = 0x0003;
pub const MBOX_WRITE_RAM_WORD: c_uint = 0x0004;
pub const MBOX_READ_RAM_WORD: c_uint = 0x0005;
pub const MBOX_MAILBOX_REG_TEST: c_uint = 0x0006;
pub const MBOX_VERIFY_CHECKSUM: c_uint = 0x0007;
pub const MBOX_ABOUT_FIRMWARE: c_uint = 0x0008;
pub const MBOX_CHECK_FIRMWARE: c_uint = 0x000e;
pub const MBOX_INIT_REQ_QUEUE: c_uint = 0x0010;
pub const MBOX_INIT_RES_QUEUE: c_uint = 0x0011;
pub const MBOX_EXECUTE_IOCB: c_uint = 0x0012;
pub const MBOX_WAKE_UP: c_uint = 0x0013;
pub const MBOX_STOP_FIRMWARE: c_uint = 0x0014;
pub const MBOX_ABORT: c_uint = 0x0015;
pub const MBOX_ABORT_DEVICE: c_uint = 0x0016;
pub const MBOX_ABORT_TARGET: c_uint = 0x0017;
pub const MBOX_BUS_RESET: c_uint = 0x0018;
pub const MBOX_STOP_QUEUE: c_uint = 0x0019;
pub const MBOX_START_QUEUE: c_uint = 0x001a;
pub const MBOX_SINGLE_STEP_QUEUE: c_uint = 0x001b;
pub const MBOX_ABORT_QUEUE: c_uint = 0x001c;
pub const MBOX_GET_DEV_QUEUE_STATUS: c_uint = 0x001d;
pub const MBOX_GET_FIRMWARE_STATUS: c_uint = 0x001f;
pub const MBOX_GET_INIT_SCSI_ID: c_uint = 0x0020;
pub const MBOX_GET_SELECT_TIMEOUT: c_uint = 0x0021;
pub const MBOX_GET_RETRY_COUNT: c_uint = 0x0022;
pub const MBOX_GET_TAG_AGE_LIMIT: c_uint = 0x0023;
pub const MBOX_GET_CLOCK_RATE: c_uint = 0x0024;
pub const MBOX_GET_ACT_NEG_STATE: c_uint = 0x0025;
pub const MBOX_GET_ASYNC_DATA_SETUP_TIME: c_uint = 0x0026;
pub const MBOX_GET_SBUS_PARAMS: c_uint = 0x0027;
pub const MBOX_GET_TARGET_PARAMS: c_uint = 0x0028;
pub const MBOX_GET_DEV_QUEUE_PARAMS: c_uint = 0x0029;
pub const MBOX_SET_INIT_SCSI_ID: c_uint = 0x0030;
pub const MBOX_SET_SELECT_TIMEOUT: c_uint = 0x0031;
pub const MBOX_SET_RETRY_COUNT: c_uint = 0x0032;
pub const MBOX_SET_TAG_AGE_LIMIT: c_uint = 0x0033;
pub const MBOX_SET_CLOCK_RATE: c_uint = 0x0034;
pub const MBOX_SET_ACTIVE_NEG_STATE: c_uint = 0x0035;
pub const MBOX_SET_ASYNC_DATA_SETUP_TIME: c_uint = 0x0036;
pub const MBOX_SET_SBUS_CONTROL_PARAMS: c_uint = 0x0037;
pub const MBOX_SET_TARGET_PARAMS: c_uint = 0x0038;
pub const MBOX_SET_DEV_QUEUE_PARAMS: c_uint = 0x0039;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_param {
    pub initiator_scsi_id: u_short,
    pub bus_reset_delay: u_short,
    pub retry_count: u_short,
    pub retry_delay: u_short,
    pub async_data_setup_time: u_short,
    pub req_ack_active_negation: u_short,
    pub data_line_active_negation: u_short,
    pub data_dma_burst_enable: u_short,
    pub command_dma_burst_enable: u_short,
    pub tag_aging: u_short,
    pub selection_timeout: u_short,
    pub max_queue_depth: u_short,
}

//
// Device Flags:
//
// Bit  Name
// ---------
// 7   Disconnect Privilege
// 6   Parity Checking
// 5   Wide Data Transfers
// 4   Synchronous Data Transfers
// 3   Tagged Queuing
// 2   Automatic Request Sense
// 1   Stop Queue on Check Condition
// 0   Renegotiate on Error
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_param {
    pub device_flags: u_short,
    pub execution_throttle: u_short,
    pub synchronous_period: u_short,
    pub synchronous_offset: u_short,
    pub device_enable: u_short,
    pub /: *mut *mut u_short reserved; / pad,
}

//
// The result queue can be quite a bit smaller since continuation entries
// do not show up there:
//

pub const QUEUE_ENTRY_LEN: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pti_queue_entry {
    pub __opaque: [c_char; QUEUE_ENTRY_LEN],
}

// Software state for the driver.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlogicpti {
// These are the hot elements in the cache, so they come first.
    pub /: *mut *mut *mut void __iomem qregs; / Adapter registers,
    pub /: *mut *mut *mut pti_queue_entry res_cpu; / Ptr to RESPONSE bufs (CPU),
    pub /: *mut *mut *mut pti_queue_entry req_cpu; / Ptr to REQUEST bufs (CPU),
    pub /: *mut *mut u_int req_in_ptr; / index of next request slot,
    pub /: *mut *mut u_int res_out_ptr; / index of next result slot,
    pub /: *mut *mut long send_marker; / must we send a marker?,
    pub op: *mut platform_device,
    pub __pad: c_ulong,
    pub cmd_count: [c_int; MAX_TARGETS],
    pub tag_ages: [c_ulong; MAX_TARGETS],
// The cmd->handler is only 32-bits, so that things work even on monster
// Ex000 sparc64 machines with >4GB of ram we just keep track of the
// scsi command pointers here.  This is essentially what Matt Jacob does. -DaveM
//
    pub 1]: *mut *mut scsi_cmnd cmd_slots[QLOGICPTI_REQ_QUEUE_LEN +,
// The rest of the elements are unimportant for performance.
    pub next: *mut qlogicpti,
    pub (DVMA)*/: *mut *mut dma_addr_t res_dvma; / Ptr to RESPONSE bufs,
    pub /: *mut *mut dma_addr_t req_dvma; / Ptr to REQUEST bufs (DVMA),
    pub fware_micrev: u_char fware_majrev, fware_minrev,,
    pub qhost: *mut Scsi_Host,
    pub qpti_id: c_int,
    pub scsi_id: c_int,
    pub prom_node: c_int,
    pub irq: c_int,
    pub clock: char differential, ultra,,
    pub bursts: c_uchar,
    pub host_param: host_param,
    pub dev_param: [dev_param; MAX_TARGETS],
    pub sreg: *mut void __iomem,
pub const SREG_TPOWER: c_uint = 0x80   /* State of termpwr           */;
pub const SREG_FUSE: c_uint = 0x40   /* State of on board fuse     */;
pub const SREG_PDISAB: c_uint = 0x20   /* Disable state for power on */;
pub const SREG_DSENSE: c_uint = 0x10   /* Sense for differential     */;
pub const SREG_IMASK: c_uint = 0x0c   /* Interrupt level            */;
pub const SREG_SPMASK: c_uint = 0x03   /* Mask for switch pack       */;
    pub swsreg: c_uchar,
    pub /: *mut *mut is_pti : 1; / Non-zero if this is a PTI board.,
}

// How to twiddle them bits...
// SBUS config register one.
pub const SBUS_CFG1_EPAR: c_uint = 0x0100      /* Enable parity checking           */;
pub const SBUS_CFG1_FMASK: c_uint = 0x00f0      /* Forth code cycle mask            */;
pub const SBUS_CFG1_BENAB: c_uint = 0x0004      /* Burst dvma enable                */;
pub const SBUS_CFG1_B64: c_uint = 0x0003      /* Enable 64byte bursts             */;
pub const SBUS_CFG1_B32: c_uint = 0x0002      /* Enable 32byte bursts             */;
pub const SBUS_CFG1_B16: c_uint = 0x0001      /* Enable 16byte bursts             */;
pub const SBUS_CFG1_B8: c_uint = 0x0008      /* Enable 8byte bursts              */;
// SBUS control register
pub const SBUS_CTRL_EDIRQ: c_uint = 0x0020      /* Enable Data DVMA Interrupts      */;
pub const SBUS_CTRL_ECIRQ: c_uint = 0x0010      /* Enable Command DVMA Interrupts   */;
pub const SBUS_CTRL_ESIRQ: c_uint = 0x0008      /* Enable SCSI Processor Interrupts */;
pub const SBUS_CTRL_ERIRQ: c_uint = 0x0004      /* Enable RISC Processor Interrupts */;
pub const SBUS_CTRL_GENAB: c_uint = 0x0002      /* Global Interrupt Enable          */;
pub const SBUS_CTRL_RESET: c_uint = 0x0001      /* Soft Reset                       */;
// SBUS status register
pub const SBUS_STAT_DINT: c_uint = 0x0020      /* Data DVMA IRQ pending            */;
pub const SBUS_STAT_CINT: c_uint = 0x0010      /* Command DVMA IRQ pending         */;
pub const SBUS_STAT_SINT: c_uint = 0x0008      /* SCSI Processor IRQ pending       */;
pub const SBUS_STAT_RINT: c_uint = 0x0004      /* RISC Processor IRQ pending       */;
pub const SBUS_STAT_GINT: c_uint = 0x0002      /* Global IRQ pending               */;
// SBUS semaphore register
pub const SBUS_SEMAPHORE_STAT: c_uint = 0x0002      /* Semaphore status bit             */;
pub const SBUS_SEMAPHORE_LCK: c_uint = 0x0001      /* Semaphore lock bit               */;
// DVMA control register
pub const DMA_CTRL_CSUSPEND: c_uint = 0x0010      /* DMA channel suspend              */;
pub const DMA_CTRL_CCLEAR: c_uint = 0x0008      /* DMA channel clear and reset      */;
pub const DMA_CTRL_FCLEAR: c_uint = 0x0004      /* DMA fifo clear                   */;
pub const DMA_CTRL_CIRQ: c_uint = 0x0002      /* DMA irq clear                    */;
pub const DMA_CTRL_DMASTART: c_uint = 0x0001      /* DMA transfer start               */;
// SCSI processor override register
pub const CPU_ORIDE_ETRIG: c_uint = 0x8000      /* External trigger enable          */;
pub const CPU_ORIDE_STEP: c_uint = 0x4000      /* Single step mode enable          */;
pub const CPU_ORIDE_BKPT: c_uint = 0x2000      /* Breakpoint reg enable            */;
pub const CPU_ORIDE_PWRITE: c_uint = 0x1000      /* SCSI pin write enable            */;
pub const CPU_ORIDE_OFORCE: c_uint = 0x0800      /* Force outputs on                 */;
pub const CPU_ORIDE_LBACK: c_uint = 0x0400      /* SCSI loopback enable             */;
pub const CPU_ORIDE_PTEST: c_uint = 0x0200      /* Parity test enable               */;
pub const CPU_ORIDE_TENAB: c_uint = 0x0100      /* SCSI pins tristate enable        */;
pub const CPU_ORIDE_TPINS: c_uint = 0x0080      /* SCSI pins enable                 */;
pub const CPU_ORIDE_FRESET: c_uint = 0x0008      /* FIFO reset                       */;
pub const CPU_ORIDE_CTERM: c_uint = 0x0004      /* Command terminate                */;
pub const CPU_ORIDE_RREG: c_uint = 0x0002      /* Reset SCSI processor regs        */;
pub const CPU_ORIDE_RMOD: c_uint = 0x0001      /* Reset SCSI processor module      */;
// SCSI processor commands
pub const CPU_CMD_BRESET: c_uint = 0x300b      /* Reset SCSI bus                   */;
// SCSI processor pin control register
pub const CPU_PCTRL_PVALID: c_uint = 0x8000      /* Phase bits are valid             */;
pub const CPU_PCTRL_PHI: c_uint = 0x0400      /* Parity bit high                  */;
pub const CPU_PCTRL_PLO: c_uint = 0x0200      /* Parity bit low                   */;
pub const CPU_PCTRL_REQ: c_uint = 0x0100      /* REQ bus signal                   */;
pub const CPU_PCTRL_ACK: c_uint = 0x0080      /* ACK bus signal                   */;
pub const CPU_PCTRL_RST: c_uint = 0x0040      /* RST bus signal                   */;
pub const CPU_PCTRL_BSY: c_uint = 0x0020      /* BSY bus signal                   */;
pub const CPU_PCTRL_SEL: c_uint = 0x0010      /* SEL bus signal                   */;
pub const CPU_PCTRL_ATN: c_uint = 0x0008      /* ATN bus signal                   */;
pub const CPU_PCTRL_MSG: c_uint = 0x0004      /* MSG bus signal                   */;
pub const CPU_PCTRL_CD: c_uint = 0x0002      /* CD bus signal                    */;
pub const CPU_PCTRL_IO: c_uint = 0x0001      /* IO bus signal                    */;
// SCSI processor differential pins register
pub const CPU_PDIFF_SENSE: c_uint = 0x0200      /* Differential sense               */;
pub const CPU_PDIFF_MODE: c_uint = 0x0100      /* Differential mode                */;
pub const CPU_PDIFF_OENAB: c_uint = 0x0080      /* Outputs enable                   */;
pub const CPU_PDIFF_PMASK: c_uint = 0x007c      /* Differential control pins        */;
pub const CPU_PDIFF_TGT: c_uint = 0x0002      /* Target mode enable               */;
pub const CPU_PDIFF_INIT: c_uint = 0x0001      /* Initiator mode enable            */;
// RISC processor status register
pub const RISC_PSR_FTRUE: c_uint = 0x8000      /* Force true                       */;
pub const RISC_PSR_LCD: c_uint = 0x4000      /* Loop counter shows done status   */;
pub const RISC_PSR_RIRQ: c_uint = 0x2000      /* RISC irq status                  */;
pub const RISC_PSR_TOFLOW: c_uint = 0x1000      /* Timer overflow (rollover)        */;
pub const RISC_PSR_AOFLOW: c_uint = 0x0800      /* Arithmetic overflow              */;
pub const RISC_PSR_AMSB: c_uint = 0x0400      /* Arithmetic big endian            */;
pub const RISC_PSR_ACARRY: c_uint = 0x0200      /* Arithmetic carry                 */;
pub const RISC_PSR_AZERO: c_uint = 0x0100      /* Arithmetic zero                  */;
pub const RISC_PSR_ULTRA: c_uint = 0x0020      /* Ultra mode                       */;
pub const RISC_PSR_DIRQ: c_uint = 0x0010      /* DVMA interrupt                   */;
pub const RISC_PSR_SIRQ: c_uint = 0x0008      /* SCSI processor interrupt         */;
pub const RISC_PSR_HIRQ: c_uint = 0x0004      /* Host interrupt                   */;
pub const RISC_PSR_IPEND: c_uint = 0x0002      /* Interrupt pending                */;
pub const RISC_PSR_FFALSE: c_uint = 0x0001      /* Force false                      */;
// RISC processor memory timing register
pub const RISC_MTREG_P1DFLT: c_uint = 0x1200      /* Default read/write timing, pg1   */;
pub const RISC_MTREG_P0DFLT: c_uint = 0x0012      /* Default read/write timing, pg0   */;
pub const RISC_MTREG_P1ULTRA: c_uint = 0x2300      /* Ultra-mode rw timing, pg1        */;
pub const RISC_MTREG_P0ULTRA: c_uint = 0x0023      /* Ultra-mode rw timing, pg0        */;
// Host command/ctrl register
pub const HCCTRL_NOP: c_uint = 0x0000      /* CMD: No operation                */;
pub const HCCTRL_RESET: c_uint = 0x1000      /* CMD: Reset RISC cpu              */;
pub const HCCTRL_PAUSE: c_uint = 0x2000      /* CMD: Pause RISC cpu              */;
pub const HCCTRL_REL: c_uint = 0x3000      /* CMD: Release paused RISC cpu     */;
pub const HCCTRL_STEP: c_uint = 0x4000      /* CMD: Single step RISC cpu        */;
pub const HCCTRL_SHIRQ: c_uint = 0x5000      /* CMD: Set host irq                */;
pub const HCCTRL_CHIRQ: c_uint = 0x6000      /* CMD: Clear host irq              */;
pub const HCCTRL_CRIRQ: c_uint = 0x7000      /* CMD: Clear RISC cpu irq          */;
pub const HCCTRL_BKPT: c_uint = 0x8000      /* CMD: Breakpoint enables change   */;
pub const HCCTRL_TMODE: c_uint = 0xf000      /* CMD: Enable test mode            */;
pub const HCCTRL_HIRQ: c_uint = 0x0080      /* Host IRQ pending                 */;
pub const HCCTRL_RRIP: c_uint = 0x0040      /* RISC cpu reset in happening now  */;
pub const HCCTRL_RPAUSED: c_uint = 0x0020      /* RISC cpu is paused now           */;
pub const HCCTRL_EBENAB: c_uint = 0x0010      /* External breakpoint enable       */;
pub const HCCTRL_B1ENAB: c_uint = 0x0008      /* Breakpoint 1 enable              */;
pub const HCCTRL_B0ENAB: c_uint = 0x0004      /* Breakpoint 0 enable              */;
// For our interrupt engine.

