//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/icom.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// icom.c
//
// Copyright (C) 2001 IBM Corporation. All rights reserved.
//
// Serial device driver.
//
// Based on code from serial.c
//

// #define ICOM_TRACE		 enable port trace capabilities

pub const NR_PORTS: c_int = 128;
    static const unsigned int icom_acfg_baud[] = {
    300,
    600,
    900,
    1200,
    1800,
    2400,
    3600,
    4800,
    7200,
    9600,
    14400,
    19200,
    28800,
    38400,
    57600,
    76800,
    115200,
    153600,
    230400,
    307200,
    460800,
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icom_regs {
    pub /: *mut *mut u32 control; / Adapter Control Register,
    pub /: *mut *mut u32 interrupt; / Adapter Interrupt Register,
    pub /: *mut *mut u32 int_mask; / Adapter Interrupt Mask Reg,
    pub /: *mut *mut u32 int_pri; / Adapter Interrupt Priority r,
    pub /: *mut *mut u32 int_reg_b; / Adapter non-masked Interrupt,
    pub resvd01: u32,
    pub resvd02: u32,
    pub resvd03: u32,
    pub /: *mut *mut u32 control_2; / Adapter Control Register 2,
    pub /: *mut *mut u32 interrupt_2; / Adapter Interrupt Register 2,
    pub /: *mut *mut u32 int_mask_2; / Adapter Interrupt Mask 2,
    pub /: *mut *mut u32 int_pri_2; / Adapter Interrupt Prior 2,
    pub /: *mut *mut u32 int_reg_2b; / Adapter non-masked 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct func_dram {
    pub /: *mut *mut u32 reserved[108]; / 0-1B0 reserved by personality code,
    pub /: *mut *mut u32 RcvStatusAddr; / 1B0-1B3 Status Address for Next rcv,
    pub /: *mut *mut u8 RcvStnAddr; / 1B4 Receive Station Addr,
    pub /: *mut *mut u8 IdleState; / 1B5 Idle State,
    pub /: *mut *mut u8 IdleMonitor; / 1B6 Idle Monitor,
    pub /: *mut *mut u8 FlagFillIdleTimer; / 1B7 Flag Fill Idle Timer,
    pub /: *mut *mut u32 XmitStatusAddr; / 1B8-1BB Transmit Status Address,
    pub /: *mut *mut u8 StartXmitCmd; / 1BC Start Xmit Command,
    pub /: *mut *mut u8 HDLCConfigReg; / 1BD Reserved,
    pub /: *mut *mut u8 CauseCode; / 1BE Cause code for fatal error,
    pub /: *mut *mut u8 xchar; / 1BF High priority send,
    pub /: *mut *mut u32 reserved3; / 1C0-1C3 Reserved,
    pub /: *mut *mut u8 PrevCmdReg; / 1C4 Reserved,
    pub /: *mut *mut u8 CmdReg; / 1C5 Command Register,
    pub /: *mut *mut u8 async_config2; / 1C6 Async Config Byte 2,
    pub /: *mut *mut u8 async_config3; / 1C7 Async Config Byte 3,
    pub /: *mut *mut u8 dce_resvd[20]; / 1C8-1DB DCE Rsvd,
    pub /: *mut *mut u8 dce_resvd21; / 1DC DCE Rsvd (21st byte,
    pub /: *mut *mut u8 misc_flags; / 1DD misc flags,
pub const V2_HARDWARE: c_uint = 0x40;
pub const ICOM_HDW_ACTIVE: c_uint = 0x01;
    pub /: *mut *mut u8 call_length; / 1DE Phone #/CFI buff ln,
    pub /: *mut *mut u8 call_length2; / 1DF Upper byte (unused),
    pub /: *mut *mut u32 call_addr; / 1E0-1E3 Phn #/CFI buff addr,
    pub /: *mut *mut u16 timer_value; / 1E4-1E5 general timer value,
    pub /: *mut *mut u8 timer_command; / 1E6 general timer cmd,
    pub /: *mut *mut u8 dce_command; / 1E7 dce command reg,
    pub /: *mut *mut u8 dce_cmd_status; / 1E8 dce command stat,
    pub /: *mut *mut u8 x21_r1_ioff; / 1E9 dce ready counter,
    pub /: *mut *mut u8 x21_r0_ioff; / 1EA dce not ready ctr,
    pub /: *mut *mut u8 x21_ralt_ioff; / 1EB dce CNR counter,
    pub /: *mut *mut u8 x21_r1_ion; / 1EC dce ready I on ctr,
    pub /: *mut *mut u8 rsvd_ier; / 1ED Rsvd for IER (if ne,
    pub /: *mut *mut u8 ier; / 1EE Interrupt Enable,
    pub /: *mut *mut u8 isr; / 1EF Input Signal Reg,
    pub /: *mut *mut u8 osr; / 1F0 Output Signal Reg,
    pub /: *mut *mut u8 reset; / 1F1 Reset/Reload Reg,
    pub /: *mut *mut u8 disable; / 1F2 Disable Reg,
    pub /: *mut *mut u8 sync; / 1F3 Sync Reg,
    pub /: *mut *mut u8 error_stat; / 1F4 Error Status,
    pub /: *mut *mut u8 cable_id; / 1F5 Cable ID,
    pub /: *mut *mut u8 cs_length; / 1F6 CS Load Length,
    pub /: *mut *mut u8 mac_length; / 1F7 Mac Load Length,
    pub /: *mut *mut u32 cs_load_addr; / 1F8-1FB Call Load PCI Addr,
    pub /: *mut *mut u32 mac_load_addr; / 1FC-1FF Mac Load PCI Addr,
}

//
// adapter defines and structures
//
pub const ICOM_CONTROL_START_A: c_uint = 0x00000008;
pub const ICOM_CONTROL_STOP_A: c_uint = 0x00000004;
pub const ICOM_CONTROL_START_B: c_uint = 0x00000002;
pub const ICOM_CONTROL_STOP_B: c_uint = 0x00000001;
pub const ICOM_CONTROL_START_C: c_uint = 0x00000008;
pub const ICOM_CONTROL_STOP_C: c_uint = 0x00000004;
pub const ICOM_CONTROL_START_D: c_uint = 0x00000002;
pub const ICOM_CONTROL_STOP_D: c_uint = 0x00000001;
pub const ICOM_IRAM_OFFSET: c_uint = 0x1000;
pub const ICOM_IRAM_SIZE: c_uint = 0x0C00;
pub const ICOM_DCE_IRAM_OFFSET: c_uint = 0x0A00;
pub const ICOM_CABLE_ID_VALID: c_uint = 0x01;
pub const ICOM_CABLE_ID_MASK: c_uint = 0xF0;
pub const ICOM_DISABLE: c_uint = 0x80;
pub const CMD_XMIT_RCV_ENABLE: c_uint = 0xC0;
pub const CMD_XMIT_ENABLE: c_uint = 0x40;
pub const CMD_RCV_DISABLE: c_uint = 0x00;
pub const CMD_RCV_ENABLE: c_uint = 0x80;
pub const CMD_RESTART: c_uint = 0x01;
pub const CMD_HOLD_XMIT: c_uint = 0x02;
pub const CMD_SND_BREAK: c_uint = 0x04;
pub const RS232_CABLE: c_uint = 0x06;
pub const V24_CABLE: c_uint = 0x0E;
pub const V35_CABLE: c_uint = 0x0C;
pub const V36_CABLE: c_uint = 0x02;
pub const NO_CABLE: c_uint = 0x00;
pub const START_DOWNLOAD: c_uint = 0x80;
pub const ICOM_INT_MASK_PRC_A: c_uint = 0x00003FFF;
pub const ICOM_INT_MASK_PRC_B: c_uint = 0x3FFF0000;
pub const ICOM_INT_MASK_PRC_C: c_uint = 0x00003FFF;
pub const ICOM_INT_MASK_PRC_D: c_uint = 0x3FFF0000;
pub const INT_RCV_COMPLETED: c_uint = 0x1000;
pub const INT_XMIT_COMPLETED: c_uint = 0x2000;
pub const INT_IDLE_DETECT: c_uint = 0x0800;
pub const INT_RCV_DISABLED: c_uint = 0x0400;
pub const INT_XMIT_DISABLED: c_uint = 0x0200;
pub const INT_RCV_XMIT_SHUTDOWN: c_uint = 0x0100;
pub const INT_FATAL_ERROR: c_uint = 0x0080;
pub const INT_CABLE_PULL: c_uint = 0x0020;
pub const INT_SIGNAL_CHANGE: c_uint = 0x0010;
pub const HDLC_PPP_PURE_ASYNC: c_uint = 0x02;
pub const HDLC_FF_FILL: c_uint = 0x00;
pub const HDLC_HDW_FLOW: c_uint = 0x01;
pub const START_XMIT: c_uint = 0x80;
pub const ICOM_ACFG_DRIVE1: c_uint = 0x20;
pub const ICOM_ACFG_NO_PARITY: c_uint = 0x00;
pub const ICOM_ACFG_PARITY_ENAB: c_uint = 0x02;
pub const ICOM_ACFG_PARITY_ODD: c_uint = 0x01;
pub const ICOM_ACFG_8BPC: c_uint = 0x00;
pub const ICOM_ACFG_7BPC: c_uint = 0x04;
pub const ICOM_ACFG_6BPC: c_uint = 0x08;
pub const ICOM_ACFG_5BPC: c_uint = 0x0C;
pub const ICOM_ACFG_1STOP_BIT: c_uint = 0x00;
pub const ICOM_ACFG_2STOP_BIT: c_uint = 0x10;
pub const ICOM_DTR: c_uint = 0x80;
pub const ICOM_RTS: c_uint = 0x40;
pub const ICOM_RI: c_uint = 0x08;
pub const ICOM_DSR: c_uint = 0x80;
pub const ICOM_DCD: c_uint = 0x20;
pub const ICOM_CTS: c_uint = 0x40;
pub const NUM_XBUFFS: c_int = 1;
pub const NUM_RBUFFS: c_int = 2;
pub const RCV_BUFF_SZ: c_uint = 0x0200;
pub const XMIT_BUFF_SZ: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statusArea {
//
// Transmit Status Area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmit_status_area {
    pub /: *mut *mut __le32 leNext; / Next entry in Little Endian on Adapter,
    pub leNextASD: __le32,
    pub /: *mut *mut __le32 leBuffer; / Buffer for entry in LE for Adapter,
    pub leLengthASD: __le16,
    pub leOffsetASD: __le16,
    pub /: *mut *mut __le16 leLength; / Length of data in segment,
    pub flags: __le16,
pub const SA_FLAGS_DONE: c_uint = 0x0080	/* Done with Segment */;
pub const SA_FLAGS_CONTINUED: c_uint = 0x8000	/* More Segments */;
pub const SA_FLAGS_IDLE: c_uint = 0x4000	/* Mark IDLE after frm */;
pub const SA_FLAGS_READY_TO_XMIT: c_uint = 0x0800;
pub const SA_FLAGS_STAT_MASK: c_uint = 0x007F;
    pub xmit: [}; NUM_XBUFFS],
//
// Receive Status Area
//
    struct {
    pub /: *mut *mut __le32 leNext; / Next entry in Little Endian on Adapter,
    pub leNextASD: __le32,
    pub /: *mut *mut __le32 leBuffer; / Buffer for entry in LE for Adapter,
    pub /: *mut *mut __le16 WorkingLength; / size of segment,
    pub reserv01: __le16,
    pub /: *mut *mut __le16 leLength; / Length of data in segment,
    pub flags: __le16,
pub const SA_FL_RCV_DONE: c_uint = 0x0010	/* Data ready */;
pub const SA_FLAGS_OVERRUN: c_uint = 0x0040;
pub const SA_FLAGS_PARITY_ERROR: c_uint = 0x0080;
pub const SA_FLAGS_FRAME_ERROR: c_uint = 0x0001;
pub const SA_FLAGS_FRAME_TRUNC: c_uint = 0x0002;
pub const SA_FLAGS_BREAK_DET: c_uint = 0x0004	/* set conditionally by device driver, not hardware */;
pub const SA_FLAGS_RCV_MASK: c_uint = 0xFFE6;
    pub rcv: [}; NUM_RBUFFS],
}

    struct icom_adapter;
pub const ICOM_MAJOR: c_int = 243;
pub const ICOM_MINOR_START: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icom_port {
    pub uart_port: uart_port,
    pub cable_id: c_uchar,
    pub read_status_mask: c_uchar,
    pub ignore_status_mask: c_uchar,
    pub int_reg: *mut *mut void __iomem,
    pub global_reg: *mut icom_regs __iomem,
    pub dram: *mut func_dram __iomem,
    pub port: c_int,
    pub statStg: *mut statusArea,
    pub statStg_pci: dma_addr_t,
    pub xmitRestart: *mut __le32,
    pub xmitRestart_pci: dma_addr_t,
    pub xmit_buf: *mut c_uchar,
    pub xmit_buf_pci: dma_addr_t,
    pub recv_buf: *mut c_uchar,
    pub recv_buf_pci: dma_addr_t,
    pub next_rcv: c_int,
    pub status: c_int,

    pub adapter: *mut icom_adapter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icom_adapter {
    pub base_addr: *mut *mut void __iomem,
    pub base_addr_pci: c_ulong,
    pub pci_dev: *mut pci_dev,
    pub port_info: [icom_port; 4],
    pub index: c_int,
    pub version: c_int,
pub const ADAPTER_V1: c_uint = 0x0001;
pub const ADAPTER_V2: c_uint = 0x0002;
    pub subsystem_id: u32,
pub const FOUR_PORT_MODEL: c_uint = 0x0252;
pub const V2_TWO_PORTS_RVX: c_uint = 0x021A;
pub const V2_ONE_PORT_RVX_ONE_PORT_IMBED_MDM: c_uint = 0x0251;
    pub numb_ports: c_int,
    pub icom_adapter_entry: list_head,
    pub kref: kref,
}

// prototype
    extern void iCom_sercons_init(void);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lookup_proc_table {
    pub global_control_reg: *mut u32 __iomem,
    pub processor_id: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lookup_int_table {
    pub global_int_mask: *mut u32 __iomem,
    pub processor_id: c_ulong,
}

    static inline struct icom_port *to_icom_port(struct uart_port *port)
    {
    return container_of(port, struct icom_port, uart_port);
    }
    static const struct pci_device_id icom_pci_table[] = {
    {
    .vendor = PCI_VENDOR_ID_IBM,
    .device = PCI_DEVICE_ID_IBM_ICOM_DEV_ID_1,
    .subvendor = PCI_ANY_ID,
    .subdevice = PCI_ANY_ID,
    .driver_data = ADAPTER_V1,
    },
    {
    .vendor = PCI_VENDOR_ID_IBM,
    .device = PCI_DEVICE_ID_IBM_ICOM_DEV_ID_2,
    .subvendor = PCI_VENDOR_ID_IBM,
    .subdevice = PCI_DEVICE_ID_IBM_ICOM_V2_TWO_PORTS_RVX,
    .driver_data = ADAPTER_V2,
    },
    {
    .vendor = PCI_VENDOR_ID_IBM,
    .device = PCI_DEVICE_ID_IBM_ICOM_DEV_ID_2,
    .subvendor = PCI_VENDOR_ID_IBM,
    .subdevice = PCI_DEVICE_ID_IBM_ICOM_V2_ONE_PORT_RVX_ONE_PORT_MDM,
    .driver_data = ADAPTER_V2,
    },
    {
    .vendor = PCI_VENDOR_ID_IBM,
    .device = PCI_DEVICE_ID_IBM_ICOM_DEV_ID_2,
    .subvendor = PCI_VENDOR_ID_IBM,
    .subdevice = PCI_DEVICE_ID_IBM_ICOM_FOUR_PORT_MODEL,
    .driver_data = ADAPTER_V2,
    },
    {
    .vendor = PCI_VENDOR_ID_IBM,
    .device = PCI_DEVICE_ID_IBM_ICOM_DEV_ID_2,
    .subvendor = PCI_VENDOR_ID_IBM,
    .subdevice = PCI_DEVICE_ID_IBM_ICOM_V2_ONE_PORT_RVX_ONE_PORT_MDM_PCIE,
    .driver_data = ADAPTER_V2,
    },
    {}
    };
    static struct lookup_proc_table start_proc[4] = {
    {core::ptr::null_mut(), ICOM_CONTROL_START_A},
    {core::ptr::null_mut(), ICOM_CONTROL_START_B},
    {core::ptr::null_mut(), ICOM_CONTROL_START_C},
    {core::ptr::null_mut(), ICOM_CONTROL_START_D}
    };
    static struct lookup_proc_table stop_proc[4] = {
    {core::ptr::null_mut(), ICOM_CONTROL_STOP_A},
    {core::ptr::null_mut(), ICOM_CONTROL_STOP_B},
    {core::ptr::null_mut(), ICOM_CONTROL_STOP_C},
    {core::ptr::null_mut(), ICOM_CONTROL_STOP_D}
    };
    static struct lookup_int_table int_mask_tbl[4] = {
    {core::ptr::null_mut(), ICOM_INT_MASK_PRC_A},
    {core::ptr::null_mut(), ICOM_INT_MASK_PRC_B},
    {core::ptr::null_mut(), ICOM_INT_MASK_PRC_C},
    {core::ptr::null_mut(), ICOM_INT_MASK_PRC_D},
    };
    MODULE_DEVICE_TABLE(pci, icom_pci_table);
    static LIST_HEAD(icom_adapter_head);
// spinlock for adapter initialization and changing adapter operations
    static DEFINE_SPINLOCK(icom_lock);

    static inline void trace(struct icom_port *icom_port, char *trace_pt,
    unsigned long trace_data)
    {
    dev_info(&icom_port.adapter.pci_dev.dev, ":%d:%s - %lx\n",
    icom_port.port, trace_pt, trace_data);
    }

    static inline void trace(struct icom_port *icom_port, char *trace_pt, unsigned long trace_data) {};

    static void icom_kref_release(struct kref *kref);
#[no_mangle]
unsafe extern "C" fn free_port_memory(icom_port: *mut icom_port) {
    static void free_port_memory(struct icom_port *icom_port)
    {
    struct pci_dev *dev = icom_port.adapter.pci_dev;
    trace(icom_port, "RET_PORT_MEM", 0);
    if (icom_port.recv_buf) {
    dma_free_coherent(&dev.dev, 4096, icom_port.recv_buf,
    icom_port.recv_buf_pci);
    icom_port.recv_buf = core::ptr::null_mut();
    }
    if (icom_port.xmit_buf) {
    dma_free_coherent(&dev.dev, 4096, icom_port.xmit_buf,
    icom_port.xmit_buf_pci);
    icom_port.xmit_buf = core::ptr::null_mut();
    }
    if (icom_port.statStg) {
    dma_free_coherent(&dev.dev, 4096, icom_port.statStg,
    icom_port.statStg_pci);
    icom_port.statStg = core::ptr::null_mut();
    }
    if (icom_port.xmitRestart) {
    dma_free_coherent(&dev.dev, 4096, icom_port.xmitRestart,
    icom_port.xmitRestart_pci);
    icom_port.xmitRestart = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn get_port_memory(icom_port: *mut icom_port) -> c_int {
    static int get_port_memory(struct icom_port *icom_port)
    {
    int index;
    unsigned long stgAddr;
    unsigned long startStgAddr;
    unsigned long offset;
    struct pci_dev *dev = icom_port.adapter.pci_dev;
    icom_port.xmit_buf =
    dma_alloc_coherent(&dev.dev, 4096, &icom_port.xmit_buf_pci,
    GFP_KERNEL);
    if (!icom_port.xmit_buf) {
    dev_err(&dev.dev, "Can not allocate Transmit buffer\n");
    return -ENOMEM;
    }
    trace(icom_port, "GET_PORT_MEM",
    (unsigned long) icom_port.xmit_buf);
    icom_port.recv_buf =
    dma_alloc_coherent(&dev.dev, 4096, &icom_port.recv_buf_pci,
    GFP_KERNEL);
    if (!icom_port.recv_buf) {
    dev_err(&dev.dev, "Can not allocate Receive buffer\n");
    free_port_memory(icom_port);
    return -ENOMEM;
    }
    trace(icom_port, "GET_PORT_MEM",
    (unsigned long) icom_port.recv_buf);
    icom_port.statStg =
    dma_alloc_coherent(&dev.dev, 4096, &icom_port.statStg_pci,
    GFP_KERNEL);
    if (!icom_port.statStg) {
    dev_err(&dev.dev, "Can not allocate Status buffer\n");
    free_port_memory(icom_port);
    return -ENOMEM;
    }
    trace(icom_port, "GET_PORT_MEM",
    (unsigned long) icom_port.statStg);
    icom_port.xmitRestart =
    dma_alloc_coherent(&dev.dev, 4096, &icom_port.xmitRestart_pci,
    GFP_KERNEL);
    if (!icom_port.xmitRestart) {
    dev_err(&dev.dev,
    "Can not allocate xmit Restart buffer\n");
    free_port_memory(icom_port);
    return -ENOMEM;
    }
// FODs: Frame Out Descriptor Queue, this is a FIFO queue that
    indicates that frames are to be transmitted
//
    stgAddr = (unsigned long) icom_port.statStg;
    for (index = 0; index < NUM_XBUFFS; index++) {
    trace(icom_port, "FOD_ADDR", stgAddr);
    stgAddr = stgAddr + sizeof(icom_port.statStg.xmit[0]);
    if (index < (NUM_XBUFFS - 1)) {
    memset(&icom_port.statStg.xmit[index], 0, sizeof(struct xmit_status_area));
    icom_port.statStg.xmit[index].leLengthASD =
    cpu_to_le16(XMIT_BUFF_SZ);
    trace(icom_port, "FOD_ADDR", stgAddr);
    trace(icom_port, "FOD_XBUFF",
    (unsigned long) icom_port.xmit_buf);
    icom_port.statStg.xmit[index].leBuffer =
    cpu_to_le32(icom_port.xmit_buf_pci);
    } else if (index == (NUM_XBUFFS - 1)) {
    memset(&icom_port.statStg.xmit[index], 0, sizeof(struct xmit_status_area));
    icom_port.statStg.xmit[index].leLengthASD =
    cpu_to_le16(XMIT_BUFF_SZ);
    trace(icom_port, "FOD_XBUFF",
    (unsigned long) icom_port.xmit_buf);
    icom_port.statStg.xmit[index].leBuffer =
    cpu_to_le32(icom_port.xmit_buf_pci);
    } else {
    memset(&icom_port.statStg.xmit[index], 0, sizeof(struct xmit_status_area));
    }
    }
// FIDs
    startStgAddr = stgAddr;
// fill in every entry, even if no buffer
    for (index = 0; index <  NUM_RBUFFS; index++) {
    trace(icom_port, "FID_ADDR", stgAddr);
    stgAddr = stgAddr + sizeof(icom_port.statStg.rcv[0]);
    icom_port.statStg.rcv[index].leLength = 0;
    icom_port.statStg.rcv[index].WorkingLength =
    cpu_to_le16(RCV_BUFF_SZ);
    if (index < (NUM_RBUFFS - 1) ) {
    offset = stgAddr - (unsigned long) icom_port.statStg;
    icom_port.statStg.rcv[index].leNext =
    cpu_to_le32(icom_port. statStg_pci + offset);
    trace(icom_port, "FID_RBUFF",
    (unsigned long) icom_port.recv_buf);
    icom_port.statStg.rcv[index].leBuffer =
    cpu_to_le32(icom_port.recv_buf_pci);
    } else if (index == (NUM_RBUFFS -1) ) {
    offset = startStgAddr - (unsigned long) icom_port.statStg;
    icom_port.statStg.rcv[index].leNext =
    cpu_to_le32(icom_port. statStg_pci + offset);
    trace(icom_port, "FID_RBUFF",
    (unsigned long) icom_port.recv_buf + 2048);
    icom_port.statStg.rcv[index].leBuffer =
    cpu_to_le32(icom_port.recv_buf_pci + 2048);
    } else {
    icom_port.statStg.rcv[index].leNext = 0;
    icom_port.statStg.rcv[index].leBuffer = 0;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stop_processor(icom_port: *mut icom_port) {
    static void stop_processor(struct icom_port *icom_port)
    {
    unsigned long temp;
    unsigned long flags;
    int port;
    spin_lock_irqsave(&icom_lock, flags);
    port = icom_port.port;
    if (port >= ARRAY_SIZE(stop_proc)) {
    dev_err(&icom_port.adapter.pci_dev.dev,
    "Invalid port assignment\n");
    goto unlock;
    }
    if (port == 0 || port == 1)
    stop_proc[port].global_control_reg = &icom_port.global_reg.control;
    else
    stop_proc[port].global_control_reg = &icom_port.global_reg.control_2;
    temp = readl(stop_proc[port].global_control_reg);
    temp = (temp & ~start_proc[port].processor_id) | stop_proc[port].processor_id;
    writel(temp, stop_proc[port].global_control_reg);
// write flush
    readl(stop_proc[port].global_control_reg);
    unlock:
    spin_unlock_irqrestore(&icom_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn start_processor(icom_port: *mut icom_port) {
    static void start_processor(struct icom_port *icom_port)
    {
    unsigned long temp;
    unsigned long flags;
    int port;
    spin_lock_irqsave(&icom_lock, flags);
    port = icom_port.port;
    if (port >= ARRAY_SIZE(start_proc)) {
    dev_err(&icom_port.adapter.pci_dev.dev,
    "Invalid port assignment\n");
    goto unlock;
    }
    if (port == 0 || port == 1)
    start_proc[port].global_control_reg = &icom_port.global_reg.control;
    else
    start_proc[port].global_control_reg = &icom_port.global_reg.control_2;
    temp = readl(start_proc[port].global_control_reg);
    temp = (temp & ~stop_proc[port].processor_id) | start_proc[port].processor_id;
    writel(temp, start_proc[port].global_control_reg);
// write flush
    readl(start_proc[port].global_control_reg);
    unlock:
    spin_unlock_irqrestore(&icom_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn load_code(icom_port: *mut icom_port) {
    static void load_code(struct icom_port *icom_port)
    {
    const struct firmware *fw;
    char __iomem *iram_ptr;
    int index;
    let mut status: c_int = 0;
    void __iomem *dram_ptr = icom_port.dram;
    dma_addr_t temp_pci;
    unsigned char *new_page = core::ptr::null_mut();
    let mut cable_id: c_uchar = NO_CABLE;
    struct pci_dev *dev = icom_port.adapter.pci_dev;
// Clear out any pending interrupts
    writew(0x3FFF, icom_port.int_reg);
    trace(icom_port, "CLEAR_INTERRUPTS", 0);
// Stop processor
    stop_processor(icom_port);
// Zero out DRAM
    memset_io(dram_ptr, 0, 512);
// Load Call Setup into Adapter
    if (request_firmware(&fw, "icom_call_setup.bin", &dev.dev) < 0) {
    dev_err(&dev.dev,"Unable to load icom_call_setup.bin firmware image\n");
    status = -1;
    goto load_code_exit;
    }
    if (fw.size > ICOM_DCE_IRAM_OFFSET) {
    dev_err(&dev.dev, "Invalid firmware image for icom_call_setup.bin found.\n");
    release_firmware(fw);
    status = -1;
    goto load_code_exit;
    }
    iram_ptr = (char __iomem *)icom_port.dram + ICOM_IRAM_OFFSET;
    for (index = 0; index < fw.size; index++)
    writeb(fw.data[index], &iram_ptr[index]);
    release_firmware(fw);
// Load Resident DCE portion of Adapter
    if (request_firmware(&fw, "icom_res_dce.bin", &dev.dev) < 0) {
    dev_err(&dev.dev,"Unable to load icom_res_dce.bin firmware image\n");
    status = -1;
    goto load_code_exit;
    }
    if (fw.size > ICOM_IRAM_SIZE) {
    dev_err(&dev.dev, "Invalid firmware image for icom_res_dce.bin found.\n");
    release_firmware(fw);
    status = -1;
    goto load_code_exit;
    }
    iram_ptr = (char __iomem *) icom_port.dram + ICOM_IRAM_OFFSET;
    for (index = ICOM_DCE_IRAM_OFFSET; index < fw.size; index++)
    writeb(fw.data[index], &iram_ptr[index]);
    release_firmware(fw);
// Set Hardware level
    if (icom_port.adapter.version == ADAPTER_V2)
    writeb(V2_HARDWARE, &(icom_port.dram.misc_flags));
// Start the processor in Adapter
    start_processor(icom_port);
    writeb((HDLC_PPP_PURE_ASYNC | HDLC_FF_FILL),
    &(icom_port.dram.HDLCConfigReg));
    writeb(0x04, &(icom_port.dram.FlagFillIdleTimer));	/* 0.5 seconds */
    writeb(0x00, &(icom_port.dram.CmdReg));
    writeb(0x10, &(icom_port.dram.async_config3));
    writeb((ICOM_ACFG_DRIVE1 | ICOM_ACFG_NO_PARITY | ICOM_ACFG_8BPC |
    ICOM_ACFG_1STOP_BIT), &(icom_port.dram.async_config2));
// Set up data in icom DRAM to indicate where personality
// code is located and its length.
//
    new_page = dma_alloc_coherent(&dev.dev, 4096, &temp_pci, GFP_KERNEL);
    if (!new_page) {
    dev_err(&dev.dev, "Can not allocate DMA buffer\n");
    status = -1;
    goto load_code_exit;
    }
    if (request_firmware(&fw, "icom_asc.bin", &dev.dev) < 0) {
    dev_err(&dev.dev,"Unable to load icom_asc.bin firmware image\n");
    status = -1;
    goto load_code_exit;
    }
    if (fw.size > ICOM_DCE_IRAM_OFFSET) {
    dev_err(&dev.dev, "Invalid firmware image for icom_asc.bin found.\n");
    release_firmware(fw);
    status = -1;
    goto load_code_exit;
    }
    for (index = 0; index < fw.size; index++)
    new_page[index] = fw.data[index];
    writeb((char) ((fw.size + 16)/16), &icom_port.dram.mac_length);
    writel(temp_pci, &icom_port.dram.mac_load_addr);
    release_firmware(fw);
// Setting the syncReg to 0x80 causes adapter to start downloading
    the personality code into adapter instruction RAM.
    Once code is loaded, it will begin executing and, based on
    information provided above, will start DMAing data from
    shared memory to adapter DRAM.
//
// the wait loop below verifies this write operation has been done
    and processed
//
    writeb(START_DOWNLOAD, &icom_port.dram.sync);
// Wait max 1 Sec for data download and processor to start
    for (index = 0; index < 10; index++) {
    msleep(100);
    if (readb(&icom_port.dram.misc_flags) & ICOM_HDW_ACTIVE)
    break;
    }
    if (index == 10)
    status = -1;
//
// check Cable ID
//
    cable_id = readb(&icom_port.dram.cable_id);
    if (cable_id & ICOM_CABLE_ID_VALID) {
// Get cable ID into the lower 4 bits (standard form)
    cable_id = (cable_id & ICOM_CABLE_ID_MASK) >> 4;
    icom_port.cable_id = cable_id;
    } else {
    dev_err(&dev.dev,"Invalid or no cable attached\n");
    icom_port.cable_id = NO_CABLE;
    }
    load_code_exit:
    if (status != 0) {
// Clear out any pending interrupts
    writew(0x3FFF, icom_port.int_reg);
// Turn off port
    writeb(ICOM_DISABLE, &(icom_port.dram.disable));
// Stop processor
    stop_processor(icom_port);
    dev_err(&icom_port.adapter.pci_dev.dev,"Port not operational\n");
    }
    if (new_page != core::ptr::null_mut())
    dma_free_coherent(&dev.dev, 4096, new_page, temp_pci);
    }
#[no_mangle]
unsafe extern "C" fn icom_startup(icom_port: *mut icom_port) -> c_int {
    static int icom_startup(struct icom_port *icom_port)
    {
    unsigned long temp;
    unsigned char cable_id, raw_cable_id;
    unsigned long flags;
    int port;
    trace(icom_port, "STARTUP", 0);
    if (!icom_port.dram) {
// should NEVER be NULL
    dev_err(&icom_port.adapter.pci_dev.dev,
    "Unusable Port, port configuration missing\n");
    return -ENODEV;
    }
//
// check Cable ID
//
    raw_cable_id = readb(&icom_port.dram.cable_id);
    trace(icom_port, "CABLE_ID", raw_cable_id);
// Get cable ID into the lower 4 bits (standard form)
    cable_id = (raw_cable_id & ICOM_CABLE_ID_MASK) >> 4;
// Check for valid Cable ID
    if (!(raw_cable_id & ICOM_CABLE_ID_VALID) ||
    (cable_id != icom_port.cable_id)) {
// reload adapter code, pick up any potential changes in cable id
    load_code(icom_port);
// still no sign of cable, error out
    raw_cable_id = readb(&icom_port.dram.cable_id);
    cable_id = (raw_cable_id & ICOM_CABLE_ID_MASK) >> 4;
    if (!(raw_cable_id & ICOM_CABLE_ID_VALID) ||
    (icom_port.cable_id == NO_CABLE))
    return -EIO;
    }
//
// Finally, clear and  enable interrupts
//
    spin_lock_irqsave(&icom_lock, flags);
    port = icom_port.port;
    if (port >= ARRAY_SIZE(int_mask_tbl)) {
    dev_err(&icom_port.adapter.pci_dev.dev,
    "Invalid port assignment\n");
    goto unlock;
    }
    if (port == 0 || port == 1)
    int_mask_tbl[port].global_int_mask = &icom_port.global_reg.int_mask;
    else
    int_mask_tbl[port].global_int_mask = &icom_port.global_reg.int_mask_2;
    if (port == 0 || port == 2)
    writew(0x00FF, icom_port.int_reg);
    else
    writew(0x3F00, icom_port.int_reg);
    temp = readl(int_mask_tbl[port].global_int_mask);
    writel(temp & ~int_mask_tbl[port].processor_id, int_mask_tbl[port].global_int_mask);
// write flush
    readl(int_mask_tbl[port].global_int_mask);
    unlock:
    spin_unlock_irqrestore(&icom_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn icom_shutdown(icom_port: *mut icom_port) {
    static void icom_shutdown(struct icom_port *icom_port)
    {
    unsigned long temp;
    unsigned char cmdReg;
    unsigned long flags;
    int port;
    spin_lock_irqsave(&icom_lock, flags);
    trace(icom_port, "SHUTDOWN", 0);
//
// disable all interrupts
//
    port = icom_port.port;
    if (port >= ARRAY_SIZE(int_mask_tbl)) {
    dev_err(&icom_port.adapter.pci_dev.dev,
    "Invalid port assignment\n");
    goto unlock;
    }
    if (port == 0 || port == 1)
    int_mask_tbl[port].global_int_mask = &icom_port.global_reg.int_mask;
    else
    int_mask_tbl[port].global_int_mask = &icom_port.global_reg.int_mask_2;
    temp = readl(int_mask_tbl[port].global_int_mask);
    writel(temp | int_mask_tbl[port].processor_id, int_mask_tbl[port].global_int_mask);
// write flush
    readl(int_mask_tbl[port].global_int_mask);
    unlock:
    spin_unlock_irqrestore(&icom_lock, flags);
//
// disable break condition
//
    cmdReg = readb(&icom_port.dram.CmdReg);
    if (cmdReg & CMD_SND_BREAK) {
    writeb(cmdReg & ~CMD_SND_BREAK, &icom_port.dram.CmdReg);
    }
    }
#[no_mangle]
unsafe extern "C" fn icom_write(port: *mut uart_port) -> c_int {
    static int icom_write(struct uart_port *port)
    {
    struct icom_port *icom_port = to_icom_port(port);
    struct tty_port *tport = &port.state.port;
    unsigned long data_count;
    unsigned char cmdReg;
    unsigned long offset;
    trace(icom_port, "WRITE", 0);
    if (le16_to_cpu(icom_port.statStg.xmit[0].flags) &
    SA_FLAGS_READY_TO_XMIT) {
    trace(icom_port, "WRITE_FULL", 0);
    return 0;
    }
    data_count = kfifo_out_peek(&tport.xmit_fifo, icom_port.xmit_buf,
    XMIT_BUFF_SZ);
    if (data_count) {
    icom_port.statStg.xmit[0].flags =
    cpu_to_le16(SA_FLAGS_READY_TO_XMIT);
    icom_port.statStg.xmit[0].leLength =
    cpu_to_le16(data_count);
    offset =
    (unsigned long) &icom_port.statStg.xmit[0] -
    (unsigned long) icom_port.statStg;
// icom_port->xmitRestart =
    cpu_to_le32(icom_port.statStg_pci + offset);
    cmdReg = readb(&icom_port.dram.CmdReg);
    writeb(cmdReg | CMD_XMIT_RCV_ENABLE,
    &icom_port.dram.CmdReg);
    writeb(START_XMIT, &icom_port.dram.StartXmitCmd);
    trace(icom_port, "WRITE_START", data_count);
// write flush
    readb(&icom_port.dram.StartXmitCmd);
    }
    return data_count;
    }
#[no_mangle]
pub unsafe extern "C" fn check_modem_status(icom_port: *mut icom_port) {
    static inline void check_modem_status(struct icom_port *icom_port)
    {
    let mut old_status: static char = 0;
    char delta_status;
    unsigned char status;
    uart_port_lock(&icom_port.uart_port);
// modem input register
    status = readb(&icom_port.dram.isr);
    trace(icom_port, "CHECK_MODEM", status);
    delta_status = status ^ old_status;
    if (delta_status) {
    if (delta_status & ICOM_RI)
    icom_port.uart_port.icount.rng++;
    if (delta_status & ICOM_DSR)
    icom_port.uart_port.icount.dsr++;
    if (delta_status & ICOM_DCD)
    uart_handle_dcd_change(&icom_port.uart_port,
    delta_status & ICOM_DCD);
    if (delta_status & ICOM_CTS)
    uart_handle_cts_change(&icom_port.uart_port,
    delta_status & ICOM_CTS);
    wake_up_interruptible(&icom_port.uart_port.state.
    port.delta_msr_wait);
    old_status = status;
    }
    uart_port_unlock(&icom_port.uart_port);
    }
#[no_mangle]
unsafe extern "C" fn xmit_interrupt(port_int_reg: u16, icom_port: *mut icom_port) {
    static void xmit_interrupt(u16 port_int_reg, struct icom_port *icom_port)
    {
    struct tty_port *tport = &icom_port.uart_port.state.port;
    u16 count;
    if (port_int_reg & (INT_XMIT_COMPLETED)) {
    trace(icom_port, "XMIT_COMPLETE", 0);
// clear buffer in use bit
    icom_port.statStg.xmit[0].flags &=
    cpu_to_le16(~SA_FLAGS_READY_TO_XMIT);
    count = le16_to_cpu(icom_port.statStg.xmit[0].leLength);
    icom_port.uart_port.icount.tx += count;
    kfifo_skip_count(&tport.xmit_fifo, count);
    if (!icom_write(&icom_port.uart_port))
// activate write queue
    uart_write_wakeup(&icom_port.uart_port);
    } else
    trace(icom_port, "XMIT_DISABLED", 0);
    }
#[no_mangle]
unsafe extern "C" fn recv_interrupt(port_int_reg: u16, icom_port: *mut icom_port) {
    static void recv_interrupt(u16 port_int_reg, struct icom_port *icom_port)
    {
    short int count, rcv_buff;
    struct tty_port *port = &icom_port.uart_port.state.port;
    u16 status;
    struct uart_icount *icount;
    unsigned long offset;
    unsigned char flag;
    trace(icom_port, "RCV_COMPLETE", 0);
    rcv_buff = icom_port.next_rcv;
    status = le16_to_cpu(icom_port.statStg.rcv[rcv_buff].flags);
    while (status & SA_FL_RCV_DONE) {
    let mut first: c_int = -1;
    trace(icom_port, "FID_STATUS", status);
    count = le16_to_cpu(icom_port.statStg.rcv[rcv_buff].leLength);
    trace(icom_port, "RCV_COUNT", count);
    trace(icom_port, "REAL_COUNT", count);
    offset = le32_to_cpu(icom_port.statStg.rcv[rcv_buff].leBuffer) -
    icom_port.recv_buf_pci;
// Block copy all but the last byte as this may have status
    if (count > 0) {
    first = icom_port.recv_buf[offset];
    tty_insert_flip_string(port, icom_port.recv_buf + offset, count - 1);
    }
    icount = &icom_port.uart_port.icount;
    icount.rx += count;
// Break detect logic
    if ((status & SA_FLAGS_FRAME_ERROR)
    && first == 0) {
    status &= ~SA_FLAGS_FRAME_ERROR;
    status |= SA_FLAGS_BREAK_DET;
    trace(icom_port, "BREAK_DET", 0);
    }
    flag = TTY_NORMAL;
    if (status &
    (SA_FLAGS_BREAK_DET | SA_FLAGS_PARITY_ERROR |
    SA_FLAGS_FRAME_ERROR | SA_FLAGS_OVERRUN)) {
    if (status & SA_FLAGS_BREAK_DET)
    icount.brk++;
    if (status & SA_FLAGS_PARITY_ERROR)
    icount.parity++;
    if (status & SA_FLAGS_FRAME_ERROR)
    icount.frame++;
    if (status & SA_FLAGS_OVERRUN)
    icount.overrun++;
//
// Now check to see if character should be
// ignored, and mask off conditions which
// should be ignored.
//
    if (status & icom_port.ignore_status_mask) {
    trace(icom_port, "IGNORE_CHAR", 0);
    goto ignore_char;
    }
    status &= icom_port.read_status_mask;
    if (status & SA_FLAGS_BREAK_DET) {
    flag = TTY_BREAK;
    } else if (status & SA_FLAGS_PARITY_ERROR) {
    trace(icom_port, "PARITY_ERROR", 0);
    flag = TTY_PARITY;
    } else if (status & SA_FLAGS_FRAME_ERROR)
    flag = TTY_FRAME;
    }
    tty_insert_flip_char(port, *(icom_port.recv_buf + offset + count - 1), flag);
    if (status & SA_FLAGS_OVERRUN)
//
// Overrun is special, since it's
// reported immediately, and doesn't
// affect the current character
//
    tty_insert_flip_char(port, 0, TTY_OVERRUN);
    ignore_char:
    icom_port.statStg.rcv[rcv_buff].flags = 0;
    icom_port.statStg.rcv[rcv_buff].leLength = 0;
    icom_port.statStg.rcv[rcv_buff].WorkingLength =
    cpu_to_le16(RCV_BUFF_SZ);
    rcv_buff++;
    if (rcv_buff == NUM_RBUFFS)
    rcv_buff = 0;
    status = le16_to_cpu(icom_port.statStg.rcv[rcv_buff].flags);
    }
    icom_port.next_rcv = rcv_buff;
    tty_flip_buffer_push(port);
    }
    static void process_interrupt(u16 port_int_reg,
    struct icom_port *icom_port)
    {
    uart_port_lock(&icom_port.uart_port);
    trace(icom_port, "INTERRUPT", port_int_reg);
    if (port_int_reg & (INT_XMIT_COMPLETED | INT_XMIT_DISABLED))
    xmit_interrupt(port_int_reg, icom_port);
    if (port_int_reg & INT_RCV_COMPLETED)
    recv_interrupt(port_int_reg, icom_port);
    uart_port_unlock(&icom_port.uart_port);
    }
#[no_mangle]
unsafe extern "C" fn icom_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t icom_interrupt(int irq, void *dev_id)
    {
    void __iomem * int_reg;
    u32 adapter_interrupts;
    u16 port_int_reg;
    struct icom_adapter *icom_adapter;
    struct icom_port *icom_port;
// find icom_port for this interrupt
    icom_adapter = (struct icom_adapter *) dev_id;
    if (icom_adapter.version == ADAPTER_V2) {
    int_reg = icom_adapter.base_addr + 0x8024;
    adapter_interrupts = readl(int_reg);
    if (adapter_interrupts & 0x00003FFF) {
// port 2 interrupt,  NOTE:  for all ADAPTER_V2, port 2 will be active
    icom_port = &icom_adapter.port_info[2];
    port_int_reg = (u16) adapter_interrupts;
    process_interrupt(port_int_reg, icom_port);
    check_modem_status(icom_port);
    }
    if (adapter_interrupts & 0x3FFF0000) {
// port 3 interrupt
    icom_port = &icom_adapter.port_info[3];
    if (icom_port.status == ICOM_PORT_ACTIVE) {
    port_int_reg =
    (u16) (adapter_interrupts >> 16);
    process_interrupt(port_int_reg, icom_port);
    check_modem_status(icom_port);
    }
    }
// Clear out any pending interrupts
    writel(adapter_interrupts, int_reg);
    int_reg = icom_adapter.base_addr + 0x8004;
    } else {
    int_reg = icom_adapter.base_addr + 0x4004;
    }
    adapter_interrupts = readl(int_reg);
    if (adapter_interrupts & 0x00003FFF) {
// port 0 interrupt, NOTE:  for all adapters, port 0 will be active
    icom_port = &icom_adapter.port_info[0];
    port_int_reg = (u16) adapter_interrupts;
    process_interrupt(port_int_reg, icom_port);
    check_modem_status(icom_port);
    }
    if (adapter_interrupts & 0x3FFF0000) {
// port 1 interrupt
    icom_port = &icom_adapter.port_info[1];
    if (icom_port.status == ICOM_PORT_ACTIVE) {
    port_int_reg = (u16) (adapter_interrupts >> 16);
    process_interrupt(port_int_reg, icom_port);
    check_modem_status(icom_port);
    }
    }
// Clear out any pending interrupts
    writel(adapter_interrupts, int_reg);
// flush the write
    adapter_interrupts = readl(int_reg);
    return IRQ_HANDLED;
    }
//
// ------------------------------------------------------------------
// Begin serial-core API
// ------------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn icom_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int icom_tx_empty(struct uart_port *port)
    {
    struct icom_port *icom_port = to_icom_port(port);
    int ret;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    if (le16_to_cpu(icom_port.statStg.xmit[0].flags) &
    SA_FLAGS_READY_TO_XMIT)
    ret = TIOCSER_TEMT;
    else
    ret = 0;
    uart_port_unlock_irqrestore(port, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn icom_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void icom_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    struct icom_port *icom_port = to_icom_port(port);
    unsigned char local_osr;
    trace(icom_port, "SET_MODEM", 0);
    local_osr = readb(&icom_port.dram.osr);
    if (mctrl & TIOCM_RTS) {
    trace(icom_port, "RAISE_RTS", 0);
    local_osr |= ICOM_RTS;
    } else {
    trace(icom_port, "LOWER_RTS", 0);
    local_osr &= ~ICOM_RTS;
    }
    if (mctrl & TIOCM_DTR) {
    trace(icom_port, "RAISE_DTR", 0);
    local_osr |= ICOM_DTR;
    } else {
    trace(icom_port, "LOWER_DTR", 0);
    local_osr &= ~ICOM_DTR;
    }
    writeb(local_osr, &icom_port.dram.osr);
    }
#[no_mangle]
unsafe extern "C" fn icom_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int icom_get_mctrl(struct uart_port *port)
    {
    struct icom_port *icom_port = to_icom_port(port);
    unsigned char status;
    unsigned int result;
    trace(icom_port, "GET_MODEM", 0);
    status = readb(&icom_port.dram.isr);
    result = ((status & ICOM_DCD) ? TIOCM_CAR : 0)
    | ((status & ICOM_RI) ? TIOCM_RNG : 0)
    | ((status & ICOM_DSR) ? TIOCM_DSR : 0)
    | ((status & ICOM_CTS) ? TIOCM_CTS : 0);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn icom_stop_tx(port: *mut uart_port) {
    static void icom_stop_tx(struct uart_port *port)
    {
    struct icom_port *icom_port = to_icom_port(port);
    unsigned char cmdReg;
    trace(icom_port, "STOP", 0);
    cmdReg = readb(&icom_port.dram.CmdReg);
    writeb(cmdReg | CMD_HOLD_XMIT, &icom_port.dram.CmdReg);
    }
#[no_mangle]
unsafe extern "C" fn icom_start_tx(port: *mut uart_port) {
    static void icom_start_tx(struct uart_port *port)
    {
    struct icom_port *icom_port = to_icom_port(port);
    unsigned char cmdReg;
    trace(icom_port, "START", 0);
    cmdReg = readb(&icom_port.dram.CmdReg);
    if ((cmdReg & CMD_HOLD_XMIT) == CMD_HOLD_XMIT)
    writeb(cmdReg & ~CMD_HOLD_XMIT,
    &icom_port.dram.CmdReg);
    icom_write(port);
    }
#[no_mangle]
unsafe extern "C" fn icom_send_xchar(port: *mut uart_port, ch: c_char) {
    static void icom_send_xchar(struct uart_port *port, char ch)
    {
    struct icom_port *icom_port = to_icom_port(port);
    unsigned char xdata;
    int index;
    unsigned long flags;
    trace(icom_port, "SEND_XCHAR", ch);
// wait .1 sec to send char
    for (index = 0; index < 10; index++) {
    uart_port_lock_irqsave(port, &flags);
    xdata = readb(&icom_port.dram.xchar);
    if (xdata == 0x00) {
    trace(icom_port, "QUICK_WRITE", 0);
    writeb(ch, &icom_port.dram.xchar);
// flush write operation
    xdata = readb(&icom_port.dram.xchar);
    uart_port_unlock_irqrestore(port, flags);
    break;
    }
    uart_port_unlock_irqrestore(port, flags);
    msleep(10);
    }
    }
#[no_mangle]
unsafe extern "C" fn icom_stop_rx(port: *mut uart_port) {
    static void icom_stop_rx(struct uart_port *port)
    {
    struct icom_port *icom_port = to_icom_port(port);
    unsigned char cmdReg;
    cmdReg = readb(&icom_port.dram.CmdReg);
    writeb(cmdReg & ~CMD_RCV_ENABLE, &icom_port.dram.CmdReg);
    }
#[no_mangle]
unsafe extern "C" fn icom_break(port: *mut uart_port, break_state: c_int) {
    static void icom_break(struct uart_port *port, int break_state)
    {
    struct icom_port *icom_port = to_icom_port(port);
    unsigned char cmdReg;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    trace(icom_port, "BREAK", 0);
    cmdReg = readb(&icom_port.dram.CmdReg);
    if (break_state == -1) {
    writeb(cmdReg | CMD_SND_BREAK, &icom_port.dram.CmdReg);
    } else {
    writeb(cmdReg & ~CMD_SND_BREAK, &icom_port.dram.CmdReg);
    }
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn icom_open(port: *mut uart_port) -> c_int {
    static int icom_open(struct uart_port *port)
    {
    struct icom_port *icom_port = to_icom_port(port);
    int retval;
    kref_get(&icom_port.adapter.kref);
    retval = icom_startup(icom_port);
    if (retval) {
    kref_put(&icom_port.adapter.kref, icom_kref_release);
    trace(icom_port, "STARTUP_ERROR", 0);
    return retval;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn icom_close(port: *mut uart_port) {
    static void icom_close(struct uart_port *port)
    {
    struct icom_port *icom_port = to_icom_port(port);
    unsigned char cmdReg;
    trace(icom_port, "CLOSE", 0);
// stop receiver
    cmdReg = readb(&icom_port.dram.CmdReg);
    writeb(cmdReg & ~CMD_RCV_ENABLE, &icom_port.dram.CmdReg);
    icom_shutdown(icom_port);
    kref_put(&icom_port.adapter.kref, icom_kref_release);
    }
    static void icom_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old_termios)
    {
    struct icom_port *icom_port = to_icom_port(port);
    int baud;
    unsigned cflag, iflag;
    char new_config2;
    let mut new_config3: c_char = 0;
    char tmp_byte;
    int index;
    int rcv_buff, xmit_buff;
    unsigned long offset;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    trace(icom_port, "CHANGE_SPEED", 0);
    cflag = termios.c_cflag;
    iflag = termios.c_iflag;
    new_config2 = ICOM_ACFG_DRIVE1;
// byte size and parity
    switch (cflag & CSIZE) {
    case CS5:		/* 5 bits/char */
    new_config2 |= ICOM_ACFG_5BPC;
    break;
    case CS6:		/* 6 bits/char */
    new_config2 |= ICOM_ACFG_6BPC;
    break;
    case CS7:		/* 7 bits/char */
    new_config2 |= ICOM_ACFG_7BPC;
    break;
    case CS8:		/* 8 bits/char */
    new_config2 |= ICOM_ACFG_8BPC;
    break;
    default:
    break;
    }
    if (cflag & CSTOPB) {
// 2 stop bits
    new_config2 |= ICOM_ACFG_2STOP_BIT;
    }
    if (cflag & PARENB) {
// parity bit enabled
    new_config2 |= ICOM_ACFG_PARITY_ENAB;
    trace(icom_port, "PARENB", 0);
    }
    if (cflag & PARODD) {
// odd parity
    new_config2 |= ICOM_ACFG_PARITY_ODD;
    trace(icom_port, "PARODD", 0);
    }
// Determine divisor based on baud rate
    baud = uart_get_baud_rate(port, termios, old_termios,
    icom_acfg_baud[0],
    icom_acfg_baud[BAUD_TABLE_LIMIT]);
    for (index = 0; index < BAUD_TABLE_LIMIT; index++) {
    if (icom_acfg_baud[index] == baud) {
    new_config3 = index;
    break;
    }
    }
    uart_update_timeout(port, cflag, baud);
// CTS flow control flag and modem status interrupts
    tmp_byte = readb(&(icom_port.dram.HDLCConfigReg));
    if (cflag & CRTSCTS)
    tmp_byte |= HDLC_HDW_FLOW;
    else
    tmp_byte &= ~HDLC_HDW_FLOW;
    writeb(tmp_byte, &(icom_port.dram.HDLCConfigReg));
//
// Set up parity check flag
//
    icom_port.read_status_mask = SA_FLAGS_OVERRUN | SA_FL_RCV_DONE;
    if (iflag & INPCK)
    icom_port.read_status_mask |=
    SA_FLAGS_FRAME_ERROR | SA_FLAGS_PARITY_ERROR;
    if ((iflag & BRKINT) || (iflag & PARMRK))
    icom_port.read_status_mask |= SA_FLAGS_BREAK_DET;
//
// Characters to ignore
//
    icom_port.ignore_status_mask = 0;
    if (iflag & IGNPAR)
    icom_port.ignore_status_mask |=
    SA_FLAGS_PARITY_ERROR | SA_FLAGS_FRAME_ERROR;
    if (iflag & IGNBRK) {
    icom_port.ignore_status_mask |= SA_FLAGS_BREAK_DET;
//
// If we're ignore parity and break indicators, ignore
// overruns too.  (For real raw support).
//
    if (iflag & IGNPAR)
    icom_port.ignore_status_mask |= SA_FLAGS_OVERRUN;
    }
//
// !!! ignore all characters if CREAD is not set
//
    if ((cflag & CREAD) == 0)
    icom_port.ignore_status_mask |= SA_FL_RCV_DONE;
// Turn off Receiver to prepare for reset
    writeb(CMD_RCV_DISABLE, &icom_port.dram.CmdReg);
    for (index = 0; index < 10; index++) {
    if (readb(&icom_port.dram.PrevCmdReg) == 0x00) {
    break;
    }
    }
// clear all current buffers of data
    for (rcv_buff = 0; rcv_buff < NUM_RBUFFS; rcv_buff++) {
    icom_port.statStg.rcv[rcv_buff].flags = 0;
    icom_port.statStg.rcv[rcv_buff].leLength = 0;
    icom_port.statStg.rcv[rcv_buff].WorkingLength =
    cpu_to_le16(RCV_BUFF_SZ);
    }
    for (xmit_buff = 0; xmit_buff < NUM_XBUFFS; xmit_buff++) {
    icom_port.statStg.xmit[xmit_buff].flags = 0;
    }
// activate changes and start xmit and receiver here
// Enable the receiver
    writeb(new_config3, &(icom_port.dram.async_config3));
    writeb(new_config2, &(icom_port.dram.async_config2));
    tmp_byte = readb(&(icom_port.dram.HDLCConfigReg));
    tmp_byte |= HDLC_PPP_PURE_ASYNC | HDLC_FF_FILL;
    writeb(tmp_byte, &(icom_port.dram.HDLCConfigReg));
    writeb(0x04, &(icom_port.dram.FlagFillIdleTimer));	/* 0.5 seconds */
    writeb(0xFF, &(icom_port.dram.ier));	/* enable modem signal interrupts */
// reset processor
    writeb(CMD_RESTART, &icom_port.dram.CmdReg);
    for (index = 0; index < 10; index++) {
    if (readb(&icom_port.dram.CmdReg) == 0x00) {
    break;
    }
    }
// Enable Transmitter and Receiver
    offset =
    (unsigned long) &icom_port.statStg.rcv[0] -
    (unsigned long) icom_port.statStg;
    writel(icom_port.statStg_pci + offset,
    &icom_port.dram.RcvStatusAddr);
    icom_port.next_rcv = 0;
// icom_port->xmitRestart = 0;
    writel(icom_port.xmitRestart_pci,
    &icom_port.dram.XmitStatusAddr);
    trace(icom_port, "XR_ENAB", 0);
    writeb(CMD_XMIT_RCV_ENABLE, &icom_port.dram.CmdReg);
    uart_port_unlock_irqrestore(port, flags);
    }
    static const char *icom_type(struct uart_port *port)
    {
    return "icom";
    }
#[no_mangle]
unsafe extern "C" fn icom_config_port(port: *mut uart_port, flags: c_int) {
    static void icom_config_port(struct uart_port *port, int flags)
    {
    port.type = PORT_ICOM;
    }
    static const struct uart_ops icom_ops = {
    .tx_empty = icom_tx_empty,
    .set_mctrl = icom_set_mctrl,
    .get_mctrl = icom_get_mctrl,
    .stop_tx = icom_stop_tx,
    .start_tx = icom_start_tx,
    .send_xchar = icom_send_xchar,
    .stop_rx = icom_stop_rx,
    .break_ctl = icom_break,
    .startup = icom_open,
    .shutdown = icom_close,
    .set_termios = icom_set_termios,
    .type = icom_type,
    .config_port = icom_config_port,
    };

    static struct uart_driver icom_uart_driver = {
    .owner = THIS_MODULE,
    .driver_name = ICOM_DRIVER_NAME,
    .dev_name = "ttyA",
    .major = ICOM_MAJOR,
    .minor = ICOM_MINOR_START,
    .nr = NR_PORTS,
    .cons = ICOM_CONSOLE,
    };
#[no_mangle]
unsafe extern "C" fn icom_init_ports(icom_adapter: *mut icom_adapter) -> c_int {
    static int icom_init_ports(struct icom_adapter *icom_adapter)
    {
    let mut subsystem_id: u32 = icom_adapter.subsystem_id;
    int i;
    struct icom_port *icom_port;
    if (icom_adapter.version == ADAPTER_V1) {
    icom_adapter.numb_ports = 2;
    for (i = 0; i < 2; i++) {
    icom_port = &icom_adapter.port_info[i];
    icom_port.port = i;
    icom_port.status = ICOM_PORT_ACTIVE;
    }
    } else {
    if (subsystem_id == PCI_DEVICE_ID_IBM_ICOM_FOUR_PORT_MODEL) {
    icom_adapter.numb_ports = 4;
    for (i = 0; i < 4; i++) {
    icom_port = &icom_adapter.port_info[i];
    icom_port.port = i;
    icom_port.status = ICOM_PORT_ACTIVE;
    }
    } else {
    icom_adapter.numb_ports = 4;
    icom_adapter.port_info[0].port = 0;
    icom_adapter.port_info[0].status = ICOM_PORT_ACTIVE;
    icom_adapter.port_info[1].status = ICOM_PORT_OFF;
    icom_adapter.port_info[2].port = 2;
    icom_adapter.port_info[2].status = ICOM_PORT_ACTIVE;
    icom_adapter.port_info[3].status = ICOM_PORT_OFF;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn icom_port_active(icom_port: *mut icom_port, icom_adapter: *mut icom_adapter, port_num: c_int) {
    static void icom_port_active(struct icom_port *icom_port, struct icom_adapter *icom_adapter, int port_num)
    {
    if (icom_adapter.version == ADAPTER_V1) {
    icom_port.global_reg = icom_adapter.base_addr + 0x4000;
    icom_port.int_reg = icom_adapter.base_addr +
    0x4004 + 2 - 2 * port_num;
    } else {
    icom_port.global_reg = icom_adapter.base_addr + 0x8000;
    if (icom_port.port < 2)
    icom_port.int_reg = icom_adapter.base_addr +
    0x8004 + 2 - 2 * icom_port.port;
    else
    icom_port.int_reg = icom_adapter.base_addr +
    0x8024 + 2 - 2 * (icom_port.port - 2);
    }
    }
#[no_mangle]
unsafe extern "C" fn icom_load_ports(icom_adapter: *mut icom_adapter) -> c_int {
    static int icom_load_ports(struct icom_adapter *icom_adapter)
    {
    struct icom_port *icom_port;
    int port_num;
    for (port_num = 0; port_num < icom_adapter.numb_ports; port_num++) {
    icom_port = &icom_adapter.port_info[port_num];
    if (icom_port.status == ICOM_PORT_ACTIVE) {
    icom_port_active(icom_port, icom_adapter, port_num);
    icom_port.dram = icom_adapter.base_addr +
    0x2000 * icom_port.port;
    icom_port.adapter = icom_adapter;
// get port memory
    if (get_port_memory(icom_port) != 0) {
    dev_err(&icom_port.adapter.pci_dev.dev,
    "Memory allocation for port FAILED\n");
    }
    }
    }
    return 0;
    }
    static int icom_alloc_adapter(struct icom_adapter
// icom_adapter_ref)
    {
    let mut adapter_count: c_int = 0;
    struct icom_adapter *icom_adapter;
    struct icom_adapter *cur_adapter_entry;
    icom_adapter = kzalloc_obj(struct icom_adapter);
    if (!icom_adapter) {
    return -ENOMEM;
    }
    list_for_each_entry(cur_adapter_entry, &icom_adapter_head,
    icom_adapter_entry) {
    if (cur_adapter_entry.index != adapter_count) {
    break;
    }
    adapter_count++;
    }
    icom_adapter.index = adapter_count;
    list_add_tail(&icom_adapter.icom_adapter_entry,
    &cur_adapter_entry.icom_adapter_entry);
// icom_adapter_ref = icom_adapter;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn icom_free_adapter(icom_adapter: *mut icom_adapter) {
    static void icom_free_adapter(struct icom_adapter *icom_adapter)
    {
    list_del(&icom_adapter.icom_adapter_entry);
    kfree(icom_adapter);
    }
#[no_mangle]
unsafe extern "C" fn icom_kref_release(kref: *mut kref) {
    static void icom_kref_release(struct kref *kref)
    {
    struct icom_adapter *icom_adapter = container_of(kref,
    struct icom_adapter, kref);
    struct icom_port *icom_port;
    int index;
    for (index = 0; index < icom_adapter.numb_ports; index++) {
    icom_port = &icom_adapter.port_info[index];
    if (icom_port.status == ICOM_PORT_ACTIVE) {
    dev_info(&icom_adapter.pci_dev.dev,
    "Device removed\n");
    uart_remove_one_port(&icom_uart_driver,
    &icom_port.uart_port);
// be sure that DTR and RTS are dropped
    writeb(0x00, &icom_port.dram.osr);
// Wait 0.1 Sec for simple Init to complete
    msleep(100);
// Stop proccessor
    stop_processor(icom_port);
    free_port_memory(icom_port);
    }
    }
    free_irq(icom_adapter.pci_dev.irq, (void *) icom_adapter);
    iounmap(icom_adapter.base_addr);
    pci_release_regions(icom_adapter.pci_dev);
    icom_free_adapter(icom_adapter);
    }
    static int icom_probe(struct pci_dev *dev,
    const struct pci_device_id *ent)
    {
    int index;
    unsigned int command_reg;
    int retval;
    struct icom_adapter *icom_adapter;
    struct icom_port *icom_port;
    retval = pci_enable_device(dev);
    if (retval) {
    dev_err(&dev.dev, "Device enable FAILED\n");
    return retval;
    }
    retval = pci_request_regions(dev, "icom");
    if (retval) {
    dev_err(&dev.dev, "pci_request_regions FAILED\n");
    pci_disable_device(dev);
    return retval;
    }
    pci_set_master(dev);
    retval = pci_read_config_dword(dev, PCI_COMMAND, &command_reg);
    if (retval) {
    dev_err(&dev.dev, "PCI Config read FAILED\n");
    retval = pcibios_err_to_errno(retval);
    goto probe_exit0;
    }
    pci_write_config_dword(dev, PCI_COMMAND,
    command_reg | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER
    | PCI_COMMAND_PARITY | PCI_COMMAND_SERR);
    if (ent.driver_data == ADAPTER_V1) {
    pci_write_config_dword(dev, 0x44, 0x8300830A);
    } else {
    pci_write_config_dword(dev, 0x44, 0x42004200);
    pci_write_config_dword(dev, 0x48, 0x42004200);
    }
    retval = icom_alloc_adapter(&icom_adapter);
    if (retval) {
    dev_err(&dev.dev, "icom_alloc_adapter FAILED\n");
    retval = -EIO;
    goto probe_exit0;
    }
    icom_adapter.base_addr_pci = pci_resource_start(dev, 0);
    icom_adapter.pci_dev = dev;
    icom_adapter.version = ent.driver_data;
    icom_adapter.subsystem_id = ent.subdevice;
    retval = icom_init_ports(icom_adapter);
    if (retval) {
    dev_err(&dev.dev, "Port configuration failed\n");
    goto probe_exit1;
    }
    icom_adapter.base_addr = pci_ioremap_bar(dev, 0);
    if (!icom_adapter.base_addr) {
    retval = -ENOMEM;
    goto probe_exit1;
    }
// save off irq and request irq line
    retval = request_irq(dev.irq, icom_interrupt, IRQF_SHARED, ICOM_DRIVER_NAME, icom_adapter);
    if (retval)
    goto probe_exit2;
    retval = icom_load_ports(icom_adapter);
    for (index = 0; index < icom_adapter.numb_ports; index++) {
    icom_port = &icom_adapter.port_info[index];
    if (icom_port.status == ICOM_PORT_ACTIVE) {
    icom_port.uart_port.irq = icom_port.adapter.pci_dev.irq;
    icom_port.uart_port.type = PORT_ICOM;
    icom_port.uart_port.iotype = UPIO_MEM;
    icom_port.uart_port.membase =
    (unsigned char __iomem *)icom_adapter.base_addr_pci;
    icom_port.uart_port.fifosize = 16;
    icom_port.uart_port.ops = &icom_ops;
    icom_port.uart_port.line =
    icom_port.port + icom_adapter.index * 4;
    if (uart_add_one_port (&icom_uart_driver, &icom_port.uart_port)) {
    icom_port.status = ICOM_PORT_OFF;
    dev_err(&dev.dev, "Device add failed\n");
    } else
    dev_info(&dev.dev, "Device added\n");
    }
    }
    kref_init(&icom_adapter.kref);
    return 0;
    probe_exit2:
    iounmap(icom_adapter.base_addr);
    probe_exit1:
    icom_free_adapter(icom_adapter);
    probe_exit0:
    pci_release_regions(dev);
    pci_disable_device(dev);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn icom_remove(dev: *mut pci_dev) {
    static void icom_remove(struct pci_dev *dev)
    {
    struct icom_adapter *icom_adapter;
    list_for_each_entry(icom_adapter, &icom_adapter_head,
    icom_adapter_entry) {
    if (icom_adapter.pci_dev == dev) {
    kref_put(&icom_adapter.kref, icom_kref_release);
    return;
    }
    }
    dev_err(&dev.dev, "Unable to find device to remove\n");
    }
    static struct pci_driver icom_pci_driver = {
    .name = ICOM_DRIVER_NAME,
    .id_table = icom_pci_table,
    .probe = icom_probe,
    .remove = icom_remove,
    };
#[no_mangle]
unsafe extern "C" fn icom_init() -> int __init {
    static int __init icom_init(void)
    {
    int ret;
    ret = uart_register_driver(&icom_uart_driver);
    if (ret)
    return ret;
    ret = pci_register_driver(&icom_pci_driver);
    if (ret < 0)
    uart_unregister_driver(&icom_uart_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn icom_exit() -> void __exit {
    static void __exit icom_exit(void)
    {
    pci_unregister_driver(&icom_pci_driver);
    uart_unregister_driver(&icom_uart_driver);
    }
    module_init(icom_init);
    module_exit(icom_exit);
    MODULE_AUTHOR("Michael Anderson <mjanders@us.ibm.com>");
    MODULE_DESCRIPTION("IBM iSeries Serial IOA driver");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("icom_call_setup.bin");
    MODULE_FIRMWARE("icom_res_dce.bin");
    MODULE_FIRMWARE("icom_asc.bin");
