//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/udc/lpc32xx_udc.c
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
// USB Gadget driver for LPC32xx
//
// Authors:
// Kevin Wells <kevin.wells@nxp.com>
// Mike James
// Roland Stigge <stigge@antcom.de>
//
// Copyright (C) 2006 Philips Semiconductors
// Copyright (C) 2009 NXP Semiconductors
// Copyright (C) 2012 Roland Stigge
//
// Note: This driver is based on original work done by Mike James for
// the LPC3180.
//

//
// USB device configuration structure
//
    typedef void (*usc_chg_event)(int);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_usbd_cfg {
    pub /: *mut *mut int vbus_drv_pol; / 0=active low drive for VBUS via ISP1301,
    pub /: *mut *mut usc_chg_event conn_chgb; / Connection change event (optional),
    pub /: *mut *mut usc_chg_event susp_chgb; / Suspend/resume event (optional),
    pub /: *mut *mut usc_chg_event rmwk_chgb; / Enable/disable remote wakeup,
}

//
// controller driver data structures
//
// 16 endpoints (not to be confused with 32 hardware endpoints)
pub const NUM_ENDPOINTS: c_int = 16;
//
// IRQ indices make reading the code a little easier
//
pub const IRQ_USB_LP: c_int = 0;
pub const IRQ_USB_HP: c_int = 1;
pub const IRQ_USB_DEVDMA: c_int = 2;
pub const IRQ_USB_ATX: c_int = 3;

// Returns the interrupt mask for the selected hardware endpoint

pub const EP_INT_TYPE: c_int = 0;
pub const EP_ISO_TYPE: c_int = 1;
pub const EP_BLK_TYPE: c_int = 2;
pub const EP_CTL_TYPE: c_int = 3;
// EP0 states

// DD (DMA Descriptor) structure, requires word alignment, this is already
// defined in the LPC32XX USB device header file, but this version is slightly
// modified to tag some work data with each DMA descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_usbd_dd_gad {
    pub dd_next_phy: u32,
    pub dd_setup: u32,
    pub dd_buffer_addr: u32,
    pub dd_status: u32,
    pub dd_iso_ps_mem_addr: u32,
    pub this_dma: u32,
    pub /: *mut *mut u32 iso_status[6]; / 5 spare,
    pub dd_next_v: u32,
}

//
// Logical endpoint structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_ep {
    pub ep: usb_ep,
    pub queue: list_head,
    pub udc: *mut lpc32xx_udc,
    pub /: *mut *mut u32 hwep_num_base; / Physical hardware EP,
    pub /: *mut *mut u32 hwep_num; / Maps to hardware endpoint,
    pub maxpacket: u32,
    pub lep: u32,
    pub is_in: bool,
    pub req_pending: bool,
    pub eptype: u32,
    pub totalints: u32,
    pub wedge: bool,
}

    enum atx_type {
    ISP1301,
    STOTG04,
    };
//
// Common UDC structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_udc {
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub isp1301_i2c_client: *mut i2c_client,
// Board and device specific
    pub board: *mut lpc32xx_usbd_cfg,
    pub udp_baseaddr: *mut void __iomem,
    pub udp_irq: [c_int; 4],
    pub usb_slv_clk: *mut clk,
// DMA support
    pub udca_v_base: *mut u32,
    pub udca_p_base: u32,
    pub dd_cache: *mut dma_pool,
// Common EP and control data
    pub enabled_devints: u32,
    pub enabled_hwepints: u32,
    pub dev_status: u32,
    pub realized_eps: u32,
// VBUS detection, pullup, and power flags
    pub vbus: u8,
    pub last_vbus: u8,
    pub pullup: c_int,
    pub poweron: c_int,
    pub atx: enum atx_type,
// Work queues related to I2C support
    pub pullup_job: work_struct,
    pub power_job: work_struct,
// USB device peripheral - various
    pub ep: [lpc32xx_ep; NUM_ENDPOINTS],
    pub enabled: bool,
    pub clocked: bool,
    pub suspended: bool,
    pub ep0state: c_int,
    pub enabled_ep_cnt: core::sync::atomic::AtomicI32,
    pub ep_disable_wait_queue: wait_queue_head_t,
}

//
// Endpoint request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_request {
    pub req: usb_request,
    pub queue: list_head,
    pub dd_desc_ptr: *mut lpc32xx_usbd_dd_gad,
    pub mapped: bool,
    pub send_zlp: bool,
}

    static inline struct lpc32xx_udc *to_udc(struct usb_gadget *g)
    {
    return container_of(g, struct lpc32xx_udc, gadget);
    }

    dev_dbg(epp.udc.dev, "%s: " fmt, __func__, ## arg)

    dev_err(epp.udc.dev, "%s: " fmt, __func__, ## arg)

    dev_info(epp.udc.dev, "%s: " fmt, __func__, ## arg)

    dev_warn(epp.udc.dev, "%s:" fmt, __func__, ## arg)

//
// USB device controller register offsets
//

// DMA support registers only below
// Set, clear, or get enabled state of the DMA request status. If
// enabled, an IN or OUT token will start a DMA transfer for the EP

// DMA UDCA head pointer

// EP DMA status, enable, and disable. This is used to specifically
// enabled or disable DMA for a specific EP

// DMA master interrupts enable and pending interrupts

// DMA end of transfer interrupt enable, disable, status

// New DD request interrupt enable, disable, status

// DMA error interrupt enable, disable, status

//
// USBD_DEVINTST/USBD_DEVINTEN/USBD_DEVINTCLR/USBD_DEVINTSET
// USBD_DEVINTPRI register definitions
//

//
// USBD_EPINTST/USBD_EPINTEN/USBD_EPINTCLR/USBD_EPINTSET
// USBD_EPINTPRI register definitions
//
// End point selection macro (RX)

// End point selection macro (TX)

//
// USBD_REEP/USBD_DMARST/USBD_DMARCLR/USBD_DMARSET/USBD_EPDMAST
// USBD_EPDMAEN/USBD_EPDMADIS
// USBD_NDDRTINTST/USBD_NDDRTINTCLR/USBD_NDDRTINTSET
// USBD_EOTINTST/USBD_EOTINTCLR/USBD_EOTINTSET
// USBD_SYSERRTINTST/USBD_SYSERRTINTCLR/USBD_SYSERRTINTSET
// register definitions
//
// Endpoint selection macro

//
// SBD_DMAINTST/USBD_DMAINTEN
//

//
// USBD_RXPLEN register definitions
//

pub const USBD_PK_LEN_MASK: c_uint = 0x3FF;
//
// USBD_CTRL register definitions
//

//
// USBD_CMDCODE register definitions
//

//
// USBD_DMARST/USBD_DMARCLR/USBD_DMARSET register definitions
//

// DD (DMA Descriptor) structure, requires word alignment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_usbd_dd {
    pub dd_next: *mut u32,
    pub dd_setup: u32,
    pub dd_buffer_addr: u32,
    pub dd_status: u32,
    pub dd_iso_ps_mem_addr: u32,
}

// dd_setup bit defines
pub const DD_SETUP_ATLE_DMA_MODE: c_uint = 0x01;
pub const DD_SETUP_NEXT_DD_VALID: c_uint = 0x04;
pub const DD_SETUP_ISO_EP: c_uint = 0x10;

// dd_status bit defines
pub const DD_STATUS_DD_RETIRED: c_uint = 0x01;
pub const DD_STATUS_STS_MASK: c_uint = 0x1E;
pub const DD_STATUS_STS_NS: c_uint = 0x00 /* Not serviced */;
pub const DD_STATUS_STS_BS: c_uint = 0x02 /* Being serviced */;
pub const DD_STATUS_STS_NC: c_uint = 0x04 /* Normal completion */;
pub const DD_STATUS_STS_DUR: c_uint = 0x06 /* Data underrun (short packet) */;
pub const DD_STATUS_STS_DOR: c_uint = 0x08 /* Data overrun */;
pub const DD_STATUS_STS_SE: c_uint = 0x12 /* System error */;
pub const DD_STATUS_PKT_VAL: c_uint = 0x20 /* Packet valid */;
pub const DD_STATUS_LSB_EX: c_uint = 0x40 /* LS byte extracted (ATLE) */;
pub const DD_STATUS_MSB_EX: c_uint = 0x80 /* MS byte extracted (ATLE) */;

//
// Protocol engine bits below
//
// Device Interrupt Bit Definitions
pub const FRAME_INT: c_uint = 0x00000001;
pub const EP_FAST_INT: c_uint = 0x00000002;
pub const EP_SLOW_INT: c_uint = 0x00000004;
pub const DEV_STAT_INT: c_uint = 0x00000008;
pub const CCEMTY_INT: c_uint = 0x00000010;
pub const CDFULL_INT: c_uint = 0x00000020;
pub const RxENDPKT_INT: c_uint = 0x00000040;
pub const TxENDPKT_INT: c_uint = 0x00000080;
pub const EP_RLZED_INT: c_uint = 0x00000100;
pub const ERR_INT: c_uint = 0x00000200;
// Rx & Tx Packet Length Definitions
pub const PKT_LNGTH_MASK: c_uint = 0x000003FF;
pub const PKT_DV: c_uint = 0x00000400;
pub const PKT_RDY: c_uint = 0x00000800;
// USB Control Definitions
pub const CTRL_RD_EN: c_uint = 0x00000001;
pub const CTRL_WR_EN: c_uint = 0x00000002;
// Command Codes
pub const CMD_SET_ADDR: c_uint = 0x00D00500;
pub const CMD_CFG_DEV: c_uint = 0x00D80500;
pub const CMD_SET_MODE: c_uint = 0x00F30500;
pub const CMD_RD_FRAME: c_uint = 0x00F50500;
pub const DAT_RD_FRAME: c_uint = 0x00F50200;
pub const CMD_RD_TEST: c_uint = 0x00FD0500;
pub const DAT_RD_TEST: c_uint = 0x00FD0200;
pub const CMD_SET_DEV_STAT: c_uint = 0x00FE0500;
pub const CMD_GET_DEV_STAT: c_uint = 0x00FE0500;
pub const DAT_GET_DEV_STAT: c_uint = 0x00FE0200;
pub const CMD_GET_ERR_CODE: c_uint = 0x00FF0500;
pub const DAT_GET_ERR_CODE: c_uint = 0x00FF0200;
pub const CMD_RD_ERR_STAT: c_uint = 0x00FB0500;
pub const DAT_RD_ERR_STAT: c_uint = 0x00FB0200;

pub const CMD_CLR_BUF: c_uint = 0x00F20500;
pub const DAT_CLR_BUF: c_uint = 0x00F20200;
pub const CMD_VALID_BUF: c_uint = 0x00FA0500;
// Device Address Register Definitions
pub const DEV_ADDR_MASK: c_uint = 0x7F;
pub const DEV_EN: c_uint = 0x80;
// Device Configure Register Definitions
pub const CONF_DVICE: c_uint = 0x01;
// Device Mode Register Definitions
pub const AP_CLK: c_uint = 0x01;
pub const INAK_CI: c_uint = 0x02;
pub const INAK_CO: c_uint = 0x04;
pub const INAK_II: c_uint = 0x08;
pub const INAK_IO: c_uint = 0x10;
pub const INAK_BI: c_uint = 0x20;
pub const INAK_BO: c_uint = 0x40;
// Device Status Register Definitions
pub const DEV_CON: c_uint = 0x01;
pub const DEV_CON_CH: c_uint = 0x02;
pub const DEV_SUS: c_uint = 0x04;
pub const DEV_SUS_CH: c_uint = 0x08;
pub const DEV_RST: c_uint = 0x10;
// Error Code Register Definitions
pub const ERR_EC_MASK: c_uint = 0x0F;
pub const ERR_EA: c_uint = 0x10;
// Error Status Register Definitions
pub const ERR_PID: c_uint = 0x01;
pub const ERR_UEPKT: c_uint = 0x02;
pub const ERR_DCRC: c_uint = 0x04;
pub const ERR_TIMOUT: c_uint = 0x08;
pub const ERR_EOP: c_uint = 0x10;
pub const ERR_B_OVRN: c_uint = 0x20;
pub const ERR_BTSTF: c_uint = 0x40;
pub const ERR_TGL: c_uint = 0x80;
// Endpoint Select Register Definitions
pub const EP_SEL_F: c_uint = 0x01;
pub const EP_SEL_ST: c_uint = 0x02;
pub const EP_SEL_STP: c_uint = 0x04;
pub const EP_SEL_PO: c_uint = 0x08;
pub const EP_SEL_EPN: c_uint = 0x10;
pub const EP_SEL_B_1_FULL: c_uint = 0x20;
pub const EP_SEL_B_2_FULL: c_uint = 0x40;
// Endpoint Status Register Definitions
pub const EP_STAT_ST: c_uint = 0x01;
pub const EP_STAT_DA: c_uint = 0x20;
pub const EP_STAT_RF_MO: c_uint = 0x40;
pub const EP_STAT_CND_ST: c_uint = 0x80;
// Clear Buffer Register Definitions
pub const CLR_BUF_PO: c_uint = 0x01;
// DMA Interrupt Bit Definitions
pub const EOT_INT: c_uint = 0x01;
pub const NDD_REQ_INT: c_uint = 0x02;
pub const SYS_ERR_INT: c_uint = 0x04;

    static const char driver_name[] = "lpc32xx_udc";
//
// proc interface support
//

    static char *epnames[] = {"INT", "ISO", "BULK", "CTRL"};
    static const char debug_filename[] = "driver/udc";
#[no_mangle]
unsafe extern "C" fn proc_ep_show(s: *mut seq_file, ep: *mut lpc32xx_ep) {
    static void proc_ep_show(struct seq_file *s, struct lpc32xx_ep *ep)
    {
    struct lpc32xx_request *req;
    seq_printf(s, "\n");
    seq_printf(s, "%12s, maxpacket %4d %3s",
    ep.ep.name, ep.ep.maxpacket,
    ep.is_in ? "in" : "out");
    seq_printf(s, " type %4s", epnames[ep.eptype]);
    seq_printf(s, " ints: %12d", ep.totalints);
    if (list_empty(&ep.queue))
    seq_printf(s, "\t(queue empty)\n");
    else {
    list_for_each_entry(req, &ep.queue, queue) {
    let mut length: u32 = req.req.actual;
    seq_printf(s, "\treq %p len %d/%d buf %p\n",
    &req.req, length,
    req.req.length, req.req.buf);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn udc_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int udc_show(struct seq_file *s, void *unused)
    {
    struct lpc32xx_udc *udc = s.private;
    struct lpc32xx_ep *ep;
    unsigned long flags;
    seq_printf(s, "%s: version %s\n", driver_name, DRIVER_VERSION);
    spin_lock_irqsave(&udc.lock, flags);
    seq_printf(s, "vbus %s, pullup %s, %s powered%s, gadget %s\n\n",
    udc.vbus ? "present" : "off",
    udc.enabled ? (udc.vbus ? "active" : "enabled") :
    "disabled",
    udc.gadget.is_selfpowered ? "self" : "VBUS",
    udc.suspended ? ", suspended" : "",
    udc.driver ? udc.driver.driver.name : "(none)");
    if (udc.enabled && udc.vbus) {
    proc_ep_show(s, &udc.ep[0]);
    list_for_each_entry(ep, &udc.gadget.ep_list, ep.ep_list)
    proc_ep_show(s, ep);
    }
    spin_unlock_irqrestore(&udc.lock, flags);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(udc);
#[no_mangle]
unsafe extern "C" fn create_debug_file(udc: *mut lpc32xx_udc) {
    static void create_debug_file(struct lpc32xx_udc *udc)
    {
    debugfs_create_file(debug_filename, 0, core::ptr::null_mut(), udc, &udc_fops);
    }
#[no_mangle]
unsafe extern "C" fn remove_debug_file(udc: *mut lpc32xx_udc) {
    static void remove_debug_file(struct lpc32xx_udc *udc)
    {
    debugfs_lookup_and_remove(debug_filename, core::ptr::null_mut());
    }

    static inline void create_debug_file(struct lpc32xx_udc *udc) {}
    static inline void remove_debug_file(struct lpc32xx_udc *udc) {}

// Primary initialization sequence for the ISP1301 transceiver
#[no_mangle]
unsafe extern "C" fn isp1301_udc_configure(udc: *mut lpc32xx_udc) {
    static void isp1301_udc_configure(struct lpc32xx_udc *udc)
    {
    u8 value;
    s32 vendor, product;
    vendor = i2c_smbus_read_word_data(udc.isp1301_i2c_client, 0x00);
    product = i2c_smbus_read_word_data(udc.isp1301_i2c_client, 0x02);
    if (vendor == 0x0483 && product == 0xa0c4)
    udc.atx = STOTG04;
// LPC32XX only supports DAT_SE0 USB mode
// This sequence is important
// Disable transparent UART mode first
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    (ISP1301_I2C_MODE_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR),
    MC1_UART_EN);
// Set full speed and SE0 mode
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    (ISP1301_I2C_MODE_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR), ~0);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_MODE_CONTROL_1, (MC1_SPEED_REG | MC1_DAT_SE0));
//
// The PSW_OE enable bit state is reversed in the ISP1301 User's Guide
//
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    (ISP1301_I2C_MODE_CONTROL_2 | ISP1301_I2C_REG_CLEAR_ADDR), ~0);
    value = MC2_BI_DI;
    if (udc.atx != STOTG04)
    value |= MC2_SPD_SUSP_CTRL;
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_MODE_CONTROL_2, value);
// Driver VBUS_DRV high or low depending on board setup
    if (udc.board.vbus_drv_pol != 0)
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1, OTG1_VBUS_DRV);
    else
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR,
    OTG1_VBUS_DRV);
// Bi-directional mode with suspend control
// Enable both pulldowns for now - the pullup will be enable when VBUS
// is detected
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    (ISP1301_I2C_OTG_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR), ~0);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1,
    (0 | OTG1_DM_PULLDOWN | OTG1_DP_PULLDOWN));
// Discharge VBUS (just in case)
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1, OTG1_VBUS_DISCHRG);
    msleep(1);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    (ISP1301_I2C_OTG_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR),
    OTG1_VBUS_DISCHRG);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_LATCH | ISP1301_I2C_REG_CLEAR_ADDR, ~0);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_FALLING | ISP1301_I2C_REG_CLEAR_ADDR, ~0);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_RISING | ISP1301_I2C_REG_CLEAR_ADDR, ~0);
    dev_info(udc.dev, "ISP1301 Vendor ID  : 0x%04x\n", vendor);
    dev_info(udc.dev, "ISP1301 Product ID : 0x%04x\n", product);
    dev_info(udc.dev, "ISP1301 Version ID : 0x%04x\n",
    i2c_smbus_read_word_data(udc.isp1301_i2c_client, 0x14));
    }
// Enables or disables the USB device pullup via the ISP1301 transceiver
#[no_mangle]
unsafe extern "C" fn isp1301_pullup_set(udc: *mut lpc32xx_udc) {
    static void isp1301_pullup_set(struct lpc32xx_udc *udc)
    {
    if (udc.pullup)
// Enable pullup for bus signalling
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1, OTG1_DP_PULLUP);
    else
// Enable pullup for bus signalling
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR,
    OTG1_DP_PULLUP);
    }
#[no_mangle]
unsafe extern "C" fn pullup_work(work: *mut work_struct) {
    static void pullup_work(struct work_struct *work)
    {
    struct lpc32xx_udc *udc =
    container_of(work, struct lpc32xx_udc, pullup_job);
    isp1301_pullup_set(udc);
    }
    static void isp1301_pullup_enable(struct lpc32xx_udc *udc, int en_pullup,
    int block)
    {
    if (en_pullup == udc.pullup)
    return;
    udc.pullup = en_pullup;
    if (block)
    isp1301_pullup_set(udc);
    else
// defer slow i2c pull up setting
    schedule_work(&udc.pullup_job);
    }

// Powers up or down the ISP1301 transceiver
#[no_mangle]
unsafe extern "C" fn isp1301_set_powerstate(udc: *mut lpc32xx_udc, enable: c_int) {
    static void isp1301_set_powerstate(struct lpc32xx_udc *udc, int enable)
    {
// There is no "global power down" register for stotg04
    if (udc.atx == STOTG04)
    return;
    if (enable != 0)
// Power up ISP1301 - this ISP1301 will automatically wakeup
    when VBUS is detected */
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_MODE_CONTROL_2 | ISP1301_I2C_REG_CLEAR_ADDR,
    MC2_GLOBAL_PWR_DN);
    else
// Power down ISP1301
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_MODE_CONTROL_2, MC2_GLOBAL_PWR_DN);
    }
#[no_mangle]
unsafe extern "C" fn power_work(work: *mut work_struct) {
    static void power_work(struct work_struct *work)
    {
    struct lpc32xx_udc *udc =
    container_of(work, struct lpc32xx_udc, power_job);
    isp1301_set_powerstate(udc, udc.poweron);
    }

//
// USB protocol engine command/data read/write helper functions
//
// Issues a single command to the USB device state machine
#[no_mangle]
unsafe extern "C" fn udc_protocol_cmd_w(udc: *mut lpc32xx_udc, cmd: u32) {
    static void udc_protocol_cmd_w(struct lpc32xx_udc *udc, u32 cmd)
    {
    let mut pass: u32 = 0;
    int to;
// EP may lock on CLRI if this read isn't done
    let mut tmp: u32 = readl(USBD_DEVINTST(udc.udp_baseaddr));
    (void) tmp;
    while (pass == 0) {
    writel(USBD_CCEMPTY, USBD_DEVINTCLR(udc.udp_baseaddr));
// Write command code
    writel(cmd, USBD_CMDCODE(udc.udp_baseaddr));
    to = 10000;
    while (((readl(USBD_DEVINTST(udc.udp_baseaddr)) &
    USBD_CCEMPTY) == 0) && (to > 0)) {
    to--;
    }
    if (to > 0)
    pass = 1;
    cpu_relax();
    }
    }
// Issues 2 commands (or command and data) to the USB device state machine
    static inline void udc_protocol_cmd_data_w(struct lpc32xx_udc *udc, u32 cmd,
    u32 data)
    {
    udc_protocol_cmd_w(udc, cmd);
    udc_protocol_cmd_w(udc, data);
    }
// Issues a single command to the USB device state machine and reads
// response data
#[no_mangle]
unsafe extern "C" fn udc_protocol_cmd_r(udc: *mut lpc32xx_udc, cmd: u32) -> u32 {
    static u32 udc_protocol_cmd_r(struct lpc32xx_udc *udc, u32 cmd)
    {
    let mut to: c_int = 1000;
// Write a command and read data from the protocol engine
    writel((USBD_CDFULL | USBD_CCEMPTY),
    USBD_DEVINTCLR(udc.udp_baseaddr));
// Write command code
    udc_protocol_cmd_w(udc, cmd);
    while ((!(readl(USBD_DEVINTST(udc.udp_baseaddr)) & USBD_CDFULL))
    && (to > 0))
    to--;
    if (!to)
    dev_dbg(udc.dev,
    "Protocol engine didn't receive response (CDFULL)\n");
    return readl(USBD_CMDDATA(udc.udp_baseaddr));
    }
//
// USB device interrupt mask support functions
//
// Enable one or more USB device interrupts
#[no_mangle]
pub unsafe extern "C" fn uda_enable_devint(udc: *mut lpc32xx_udc, devmask: u32) {
    static inline void uda_enable_devint(struct lpc32xx_udc *udc, u32 devmask)
    {
    udc.enabled_devints |= devmask;
    writel(udc.enabled_devints, USBD_DEVINTEN(udc.udp_baseaddr));
    }
// Disable one or more USB device interrupts
#[no_mangle]
pub unsafe extern "C" fn uda_disable_devint(udc: *mut lpc32xx_udc, mask: u32) {
    static inline void uda_disable_devint(struct lpc32xx_udc *udc, u32 mask)
    {
    udc.enabled_devints &= ~mask;
    writel(udc.enabled_devints, USBD_DEVINTEN(udc.udp_baseaddr));
    }
// Clear one or more USB device interrupts
#[no_mangle]
pub unsafe extern "C" fn uda_clear_devint(udc: *mut lpc32xx_udc, mask: u32) {
    static inline void uda_clear_devint(struct lpc32xx_udc *udc, u32 mask)
    {
    writel(mask, USBD_DEVINTCLR(udc.udp_baseaddr));
    }
//
// Endpoint interrupt disable/enable functions
//
// Enable one or more USB endpoint interrupts
#[no_mangle]
unsafe extern "C" fn uda_enable_hwepint(udc: *mut lpc32xx_udc, hwep: u32) {
    static void uda_enable_hwepint(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc.enabled_hwepints |= (1 << hwep);
    writel(udc.enabled_hwepints, USBD_EPINTEN(udc.udp_baseaddr));
    }
// Disable one or more USB endpoint interrupts
#[no_mangle]
unsafe extern "C" fn uda_disable_hwepint(udc: *mut lpc32xx_udc, hwep: u32) {
    static void uda_disable_hwepint(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc.enabled_hwepints &= ~(1 << hwep);
    writel(udc.enabled_hwepints, USBD_EPINTEN(udc.udp_baseaddr));
    }
// Clear one or more USB endpoint interrupts
#[no_mangle]
pub unsafe extern "C" fn uda_clear_hwepint(udc: *mut lpc32xx_udc, hwep: u32) {
    static inline void uda_clear_hwepint(struct lpc32xx_udc *udc, u32 hwep)
    {
    writel((1 << hwep), USBD_EPINTCLR(udc.udp_baseaddr));
    }
// Enable DMA for the HW channel
#[no_mangle]
pub unsafe extern "C" fn udc_ep_dma_enable(udc: *mut lpc32xx_udc, hwep: u32) {
    static inline void udc_ep_dma_enable(struct lpc32xx_udc *udc, u32 hwep)
    {
    writel((1 << hwep), USBD_EPDMAEN(udc.udp_baseaddr));
    }
// Disable DMA for the HW channel
#[no_mangle]
pub unsafe extern "C" fn udc_ep_dma_disable(udc: *mut lpc32xx_udc, hwep: u32) {
    static inline void udc_ep_dma_disable(struct lpc32xx_udc *udc, u32 hwep)
    {
    writel((1 << hwep), USBD_EPDMADIS(udc.udp_baseaddr));
    }
//
// Endpoint realize/unrealize functions
//
// Before an endpoint can be used, it needs to be realized
// in the USB protocol engine - this realizes the endpoint.
// The interrupt (FIFO or DMA) is not enabled with this function
    static void udc_realize_hwep(struct lpc32xx_udc *udc, u32 hwep,
    u32 maxpacket)
    {
    let mut to: c_int = 1000;
    writel(USBD_EP_RLZED, USBD_DEVINTCLR(udc.udp_baseaddr));
    writel(hwep, USBD_EPIND(udc.udp_baseaddr));
    udc.realized_eps |= (1 << hwep);
    writel(udc.realized_eps, USBD_REEP(udc.udp_baseaddr));
    writel(maxpacket, USBD_EPMAXPSIZE(udc.udp_baseaddr));
// Wait until endpoint is realized in hardware
    while ((!(readl(USBD_DEVINTST(udc.udp_baseaddr)) &
    USBD_EP_RLZED)) && (to > 0))
    to--;
    if (!to)
    dev_dbg(udc.dev, "EP not correctly realized in hardware\n");
    writel(USBD_EP_RLZED, USBD_DEVINTCLR(udc.udp_baseaddr));
    }
// Unrealize an EP
#[no_mangle]
unsafe extern "C" fn udc_unrealize_hwep(udc: *mut lpc32xx_udc, hwep: u32) {
    static void udc_unrealize_hwep(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc.realized_eps &= ~(1 << hwep);
    writel(udc.realized_eps, USBD_REEP(udc.udp_baseaddr));
    }
//
// Endpoint support functions
//
// Select and clear endpoint interrupt
#[no_mangle]
unsafe extern "C" fn udc_selep_clrint(udc: *mut lpc32xx_udc, hwep: u32) -> u32 {
    static u32 udc_selep_clrint(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc_protocol_cmd_w(udc, CMD_SEL_EP_CLRI(hwep));
    return udc_protocol_cmd_r(udc, DAT_SEL_EP_CLRI(hwep));
    }
// Disables the endpoint in the USB protocol engine
#[no_mangle]
unsafe extern "C" fn udc_disable_hwep(udc: *mut lpc32xx_udc, hwep: u32) {
    static void udc_disable_hwep(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc_protocol_cmd_data_w(udc, CMD_SET_EP_STAT(hwep),
    DAT_WR_BYTE(EP_STAT_DA));
    }
// Stalls the endpoint - endpoint will return STALL
#[no_mangle]
unsafe extern "C" fn udc_stall_hwep(udc: *mut lpc32xx_udc, hwep: u32) {
    static void udc_stall_hwep(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc_protocol_cmd_data_w(udc, CMD_SET_EP_STAT(hwep),
    DAT_WR_BYTE(EP_STAT_ST));
    }
// Clear stall or reset endpoint
#[no_mangle]
unsafe extern "C" fn udc_clrstall_hwep(udc: *mut lpc32xx_udc, hwep: u32) {
    static void udc_clrstall_hwep(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc_protocol_cmd_data_w(udc, CMD_SET_EP_STAT(hwep),
    DAT_WR_BYTE(0));
    }
// Select an endpoint for endpoint status, clear, validate
#[no_mangle]
unsafe extern "C" fn udc_select_hwep(udc: *mut lpc32xx_udc, hwep: u32) {
    static void udc_select_hwep(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc_protocol_cmd_w(udc, CMD_SEL_EP(hwep));
    }
//
// Endpoint buffer management functions
//
// Clear the current endpoint's buffer
#[no_mangle]
unsafe extern "C" fn udc_clr_buffer_hwep(udc: *mut lpc32xx_udc, hwep: u32) {
    static void udc_clr_buffer_hwep(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc_select_hwep(udc, hwep);
    udc_protocol_cmd_w(udc, CMD_CLR_BUF);
    }
// Validate the current endpoint's buffer
#[no_mangle]
unsafe extern "C" fn udc_val_buffer_hwep(udc: *mut lpc32xx_udc, hwep: u32) {
    static void udc_val_buffer_hwep(struct lpc32xx_udc *udc, u32 hwep)
    {
    udc_select_hwep(udc, hwep);
    udc_protocol_cmd_w(udc, CMD_VALID_BUF);
    }
#[no_mangle]
pub unsafe extern "C" fn udc_clearep_getsts(udc: *mut lpc32xx_udc, hwep: u32) -> u32 {
    static inline u32 udc_clearep_getsts(struct lpc32xx_udc *udc, u32 hwep)
    {
// Clear EP interrupt
    uda_clear_hwepint(udc, hwep);
    return udc_selep_clrint(udc, hwep);
    }
//
// USB EP DMA support
//
// Allocate a DMA Descriptor
    static struct lpc32xx_usbd_dd_gad *udc_dd_alloc(struct lpc32xx_udc *udc)
    {
    dma_addr_t			dma;
    struct lpc32xx_usbd_dd_gad	*dd;
    dd = dma_pool_alloc(udc.dd_cache, GFP_ATOMIC | GFP_DMA, &dma);
    if (dd)
    dd.this_dma = dma;
    return dd;
    }
// Free a DMA Descriptor
#[no_mangle]
unsafe extern "C" fn udc_dd_free(udc: *mut lpc32xx_udc, dd: *mut lpc32xx_usbd_dd_gad) {
    static void udc_dd_free(struct lpc32xx_udc *udc, struct lpc32xx_usbd_dd_gad *dd)
    {
    dma_pool_free(udc.dd_cache, dd, dd.this_dma);
    }
//
// USB setup and shutdown functions
//
// Enables or disables most of the USB system clocks when low power mode is
// needed. Clocks are typically started on a connection event, and disabled
// when a cable is disconnected
#[no_mangle]
unsafe extern "C" fn udc_clk_set(udc: *mut lpc32xx_udc, enable: c_int) {
    static void udc_clk_set(struct lpc32xx_udc *udc, int enable)
    {
    if (enable != 0) {
    if (udc.clocked)
    return;
    udc.clocked = 1;
    clk_prepare_enable(udc.usb_slv_clk);
    } else {
    if (!udc.clocked)
    return;
    udc.clocked = 0;
    clk_disable_unprepare(udc.usb_slv_clk);
    }
    }
// Set/reset USB device address
#[no_mangle]
unsafe extern "C" fn udc_set_address(udc: *mut lpc32xx_udc, addr: u32) {
    static void udc_set_address(struct lpc32xx_udc *udc, u32 addr)
    {
// Address will be latched at the end of the status phase, or
    latched immediately if function is called twice */
    udc_protocol_cmd_data_w(udc, CMD_SET_ADDR,
    DAT_WR_BYTE(DEV_EN | addr));
    }
// Setup up a IN request for DMA transfer - this consists of determining the
// list of DMA addresses for the transfer, allocating DMA Descriptors,
// installing the DD into the UDCA, and then enabling the DMA for that EP
#[no_mangle]
unsafe extern "C" fn udc_ep_in_req_dma(udc: *mut lpc32xx_udc, ep: *mut lpc32xx_ep) -> c_int {
    static int udc_ep_in_req_dma(struct lpc32xx_udc *udc, struct lpc32xx_ep *ep)
    {
    struct lpc32xx_request *req;
    let mut hwep: u32 = ep.hwep_num;
    ep.req_pending = 1;
// There will always be a request waiting here
    req = list_entry(ep.queue.next, struct lpc32xx_request, queue);
// Place the DD Descriptor into the UDCA
    udc.udca_v_base[hwep] = req.dd_desc_ptr.this_dma;
// Enable DMA and interrupt for the HW EP
    udc_ep_dma_enable(udc, hwep);
// Clear ZLP if last packet is not of MAXP size
    if (req.req.length % ep.ep.maxpacket)
    req.send_zlp = 0;
    return 0;
    }
// Setup up a OUT request for DMA transfer - this consists of determining the
// list of DMA addresses for the transfer, allocating DMA Descriptors,
// installing the DD into the UDCA, and then enabling the DMA for that EP
#[no_mangle]
unsafe extern "C" fn udc_ep_out_req_dma(udc: *mut lpc32xx_udc, ep: *mut lpc32xx_ep) -> c_int {
    static int udc_ep_out_req_dma(struct lpc32xx_udc *udc, struct lpc32xx_ep *ep)
    {
    struct lpc32xx_request *req;
    let mut hwep: u32 = ep.hwep_num;
    ep.req_pending = 1;
// There will always be a request waiting here
    req = list_entry(ep.queue.next, struct lpc32xx_request, queue);
// Place the DD Descriptor into the UDCA
    udc.udca_v_base[hwep] = req.dd_desc_ptr.this_dma;
// Enable DMA and interrupt for the HW EP
    udc_ep_dma_enable(udc, hwep);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udc_disable(udc: *mut lpc32xx_udc) {
    static void udc_disable(struct lpc32xx_udc *udc)
    {
    u32 i;
// Disable device
    udc_protocol_cmd_data_w(udc, CMD_CFG_DEV, DAT_WR_BYTE(0));
    udc_protocol_cmd_data_w(udc, CMD_SET_DEV_STAT, DAT_WR_BYTE(0));
// Disable all device interrupts (including EP0)
    uda_disable_devint(udc, 0x3FF);
// Disable and reset all endpoint interrupts
    for (i = 0; i < 32; i++) {
    uda_disable_hwepint(udc, i);
    uda_clear_hwepint(udc, i);
    udc_disable_hwep(udc, i);
    udc_unrealize_hwep(udc, i);
    udc.udca_v_base[i] = 0;
// Disable and clear all interrupts and DMA
    udc_ep_dma_disable(udc, i);
    writel((1 << i), USBD_EOTINTCLR(udc.udp_baseaddr));
    writel((1 << i), USBD_NDDRTINTCLR(udc.udp_baseaddr));
    writel((1 << i), USBD_SYSERRTINTCLR(udc.udp_baseaddr));
    writel((1 << i), USBD_DMARCLR(udc.udp_baseaddr));
    }
// Disable DMA interrupts
    writel(0, USBD_DMAINTEN(udc.udp_baseaddr));
    writel(0, USBD_UDCAH(udc.udp_baseaddr));
    }
#[no_mangle]
unsafe extern "C" fn udc_enable(udc: *mut lpc32xx_udc) {
    static void udc_enable(struct lpc32xx_udc *udc)
    {
    u32 i;
    struct lpc32xx_ep *ep = &udc.ep[0];
// Start with known state
    udc_disable(udc);
// Enable device
    udc_protocol_cmd_data_w(udc, CMD_SET_DEV_STAT, DAT_WR_BYTE(DEV_CON));
// EP interrupts on high priority, FRAME interrupt on low priority
    writel(USBD_EP_FAST, USBD_DEVINTPRI(udc.udp_baseaddr));
    writel(0xFFFF, USBD_EPINTPRI(udc.udp_baseaddr));
// Clear any pending device interrupts
    writel(0x3FF, USBD_DEVINTCLR(udc.udp_baseaddr));
// Setup UDCA - not yet used (DMA)
    writel(udc.udca_p_base, USBD_UDCAH(udc.udp_baseaddr));
// Only enable EP0 in and out for now, EP0 only works in FIFO mode
    for (i = 0; i <= 1; i++) {
    udc_realize_hwep(udc, i, ep.ep.maxpacket);
    uda_enable_hwepint(udc, i);
    udc_select_hwep(udc, i);
    udc_clrstall_hwep(udc, i);
    udc_clr_buffer_hwep(udc, i);
    }
// Device interrupt setup
    uda_clear_devint(udc, (USBD_ERR_INT | USBD_DEV_STAT | USBD_EP_SLOW |
    USBD_EP_FAST));
    uda_enable_devint(udc, (USBD_ERR_INT | USBD_DEV_STAT | USBD_EP_SLOW |
    USBD_EP_FAST));
// Set device address to 0 - called twice to force a latch in the USB
    engine without the need of a setup packet status closure */
    udc_set_address(udc, 0);
    udc_set_address(udc, 0);
// Enable master DMA interrupts
    writel((USBD_SYS_ERR_INT | USBD_EOT_INT),
    USBD_DMAINTEN(udc.udp_baseaddr));
    udc.dev_status = 0;
    }
//
// USB device board specific events handled via callbacks
//
// Connection change event - notify board function of change
#[no_mangle]
unsafe extern "C" fn uda_power_event(udc: *mut lpc32xx_udc, conn: u32) {
    static void uda_power_event(struct lpc32xx_udc *udc, u32 conn)
    {
// Just notify of a connection change event (optional)
    if (udc.board.conn_chgb != core::ptr::null_mut())
    udc.board.conn_chgb(conn);
    }
// Suspend/resume event - notify board function of change
#[no_mangle]
unsafe extern "C" fn uda_resm_susp_event(udc: *mut lpc32xx_udc, conn: u32) {
    static void uda_resm_susp_event(struct lpc32xx_udc *udc, u32 conn)
    {
// Just notify of a Suspend/resume change event (optional)
    if (udc.board.susp_chgb != core::ptr::null_mut())
    udc.board.susp_chgb(conn);
    if (conn)
    udc.suspended = 0;
    else
    udc.suspended = 1;
    }
// Remote wakeup enable/disable - notify board function of change
#[no_mangle]
unsafe extern "C" fn uda_remwkp_cgh(udc: *mut lpc32xx_udc) {
    static void uda_remwkp_cgh(struct lpc32xx_udc *udc)
    {
    if (udc.board.rmwk_chgb != core::ptr::null_mut())
    udc.board.rmwk_chgb(udc.dev_status &
    (1 << USB_DEVICE_REMOTE_WAKEUP));
    }
// Reads data from FIFO, adjusts for alignment and data size
#[no_mangle]
unsafe extern "C" fn udc_pop_fifo(udc: *mut lpc32xx_udc, data: *mut u8, bytes: u32) {
    static void udc_pop_fifo(struct lpc32xx_udc *udc, u8 *data, u32 bytes)
    {
    int n, i, bl;
    u16 *p16;
    u32 *p32, tmp, cbytes;
// Use optimal data transfer method based on source address and size
    switch (((uintptr_t) data) & 0x3) {
    case 0: /* 32-bit aligned */
    p32 = (u32 *) data;
    cbytes = (bytes & ~0x3);
// Copy 32-bit aligned data first
    for (n = 0; n < cbytes; n += 4)
// p32++ = readl(USBD_RXDATA(udc->udp_baseaddr));
// Handle any remaining bytes
    bl = bytes - cbytes;
    if (bl) {
    tmp = readl(USBD_RXDATA(udc.udp_baseaddr));
    for (n = 0; n < bl; n++)
    data[cbytes + n] = ((tmp >> (n * 8)) & 0xFF);
    }
    break;
    case 1: /* 8-bit aligned */
    case 3:
// Each byte has to be handled independently
    for (n = 0; n < bytes; n += 4) {
    tmp = readl(USBD_RXDATA(udc.udp_baseaddr));
    bl = bytes - n;
    if (bl > 4)
    bl = 4;
    for (i = 0; i < bl; i++)
    data[n + i] = (u8) ((tmp >> (i * 8)) & 0xFF);
    }
    break;
    case 2: /* 16-bit aligned */
    p16 = (u16 *) data;
    cbytes = (bytes & ~0x3);
// Copy 32-bit sized objects first with 16-bit alignment
    for (n = 0; n < cbytes; n += 4) {
    tmp = readl(USBD_RXDATA(udc.udp_baseaddr));
// p16++ = (u16)(tmp & 0xFFFF);
// p16++ = (u16)((tmp >> 16) & 0xFFFF);
    }
// Handle any remaining bytes
    bl = bytes - cbytes;
    if (bl) {
    tmp = readl(USBD_RXDATA(udc.udp_baseaddr));
    for (n = 0; n < bl; n++)
    data[cbytes + n] = ((tmp >> (n * 8)) & 0xFF);
    }
    break;
    }
    }
// Read data from the FIFO for an endpoint. This function is for endpoints (such
// as EP0) that don't use DMA. This function should only be called if a packet
// is known to be ready to read for the endpoint. Note that the endpoint must
// be selected in the protocol engine prior to this call.
    static u32 udc_read_hwep(struct lpc32xx_udc *udc, u32 hwep, u32 *data,
    u32 bytes)
    {
    u32 tmpv;
    let mut to: c_int = 1000;
    u32 tmp, hwrep = ((hwep & 0x1E) << 1) | CTRL_RD_EN;
// Setup read of endpoint
    writel(hwrep, USBD_CTRL(udc.udp_baseaddr));
// Wait until packet is ready
    while ((((tmpv = readl(USBD_RXPLEN(udc.udp_baseaddr))) &
    PKT_RDY) == 0)	&& (to > 0))
    to--;
    if (!to)
    dev_dbg(udc.dev, "No packet ready on FIFO EP read\n");
// Mask out count
    tmp = tmpv & PKT_LNGTH_MASK;
    if (bytes < tmp)
    tmp = bytes;
    if ((tmp > 0) && (data != core::ptr::null_mut()))
    udc_pop_fifo(udc, (u8 *) data, tmp);
    writel(((hwep & 0x1E) << 1), USBD_CTRL(udc.udp_baseaddr));
// Clear the buffer
    udc_clr_buffer_hwep(udc, hwep);
    return tmp;
    }
// Stuffs data into the FIFO, adjusts for alignment and data size
#[no_mangle]
unsafe extern "C" fn udc_stuff_fifo(udc: *mut lpc32xx_udc, data: *mut u8, bytes: u32) {
    static void udc_stuff_fifo(struct lpc32xx_udc *udc, u8 *data, u32 bytes)
    {
    int n, i, bl;
    u16 *p16;
    u32 *p32, tmp, cbytes;
// Use optimal data transfer method based on source address and size
    switch (((uintptr_t) data) & 0x3) {
    case 0: /* 32-bit aligned */
    p32 = (u32 *) data;
    cbytes = (bytes & ~0x3);
// Copy 32-bit aligned data first
    for (n = 0; n < cbytes; n += 4)
    writel(*p32++, USBD_TXDATA(udc.udp_baseaddr));
// Handle any remaining bytes
    bl = bytes - cbytes;
    if (bl) {
    tmp = 0;
    for (n = 0; n < bl; n++)
    tmp |= data[cbytes + n] << (n * 8);
    writel(tmp, USBD_TXDATA(udc.udp_baseaddr));
    }
    break;
    case 1: /* 8-bit aligned */
    case 3:
// Each byte has to be handled independently
    for (n = 0; n < bytes; n += 4) {
    bl = bytes - n;
    if (bl > 4)
    bl = 4;
    tmp = 0;
    for (i = 0; i < bl; i++)
    tmp |= data[n + i] << (i * 8);
    writel(tmp, USBD_TXDATA(udc.udp_baseaddr));
    }
    break;
    case 2: /* 16-bit aligned */
    p16 = (u16 *) data;
    cbytes = (bytes & ~0x3);
// Copy 32-bit aligned data first
    for (n = 0; n < cbytes; n += 4) {
    tmp = *p16++ & 0xFFFF;
    tmp |= (*p16++ & 0xFFFF) << 16;
    writel(tmp, USBD_TXDATA(udc.udp_baseaddr));
    }
// Handle any remaining bytes
    bl = bytes - cbytes;
    if (bl) {
    tmp = 0;
    for (n = 0; n < bl; n++)
    tmp |= data[cbytes + n] << (n * 8);
    writel(tmp, USBD_TXDATA(udc.udp_baseaddr));
    }
    break;
    }
    }
// Write data to the FIFO for an endpoint. This function is for endpoints (such
// as EP0) that don't use DMA. Note that the endpoint must be selected in the
// protocol engine prior to this call.
    static void udc_write_hwep(struct lpc32xx_udc *udc, u32 hwep, u32 *data,
    u32 bytes)
    {
    let mut hwwep: u32 = ((hwep & 0x1E) << 1) | CTRL_WR_EN;
    if ((bytes > 0) && (data == core::ptr::null_mut()))
    return;
// Setup write of endpoint
    writel(hwwep, USBD_CTRL(udc.udp_baseaddr));
    writel(bytes, USBD_TXPLEN(udc.udp_baseaddr));
// Need at least 1 byte to trigger TX
    if (bytes == 0)
    writel(0, USBD_TXDATA(udc.udp_baseaddr));
    else
    udc_stuff_fifo(udc, (u8 *) data, bytes);
    writel(((hwep & 0x1E) << 1), USBD_CTRL(udc.udp_baseaddr));
    udc_val_buffer_hwep(udc, hwep);
    }
// USB device reset - resets USB to a default state with just EP0
    enabled */
#[no_mangle]
unsafe extern "C" fn uda_usb_reset(udc: *mut lpc32xx_udc) {
    static void uda_usb_reset(struct lpc32xx_udc *udc)
    {
    let mut i: u32 = 0;
// Re-init device controller and EP0
    udc_enable(udc);
    udc.gadget.speed = USB_SPEED_FULL;
    for (i = 1; i < NUM_ENDPOINTS; i++) {
    struct lpc32xx_ep *ep = &udc.ep[i];
    ep.req_pending = 0;
    }
    }
// Send a ZLP on EP0
#[no_mangle]
unsafe extern "C" fn udc_ep0_send_zlp(udc: *mut lpc32xx_udc) {
    static void udc_ep0_send_zlp(struct lpc32xx_udc *udc)
    {
    udc_write_hwep(udc, EP_IN, core::ptr::null_mut(), 0);
    }
// Get current frame number
#[no_mangle]
unsafe extern "C" fn udc_get_current_frame(udc: *mut lpc32xx_udc) -> u16 {
    static u16 udc_get_current_frame(struct lpc32xx_udc *udc)
    {
    u16 flo, fhi;
    udc_protocol_cmd_w(udc, CMD_RD_FRAME);
    flo = (u16) udc_protocol_cmd_r(udc, DAT_RD_FRAME);
    fhi = (u16) udc_protocol_cmd_r(udc, DAT_RD_FRAME);
    return (fhi << 8) | flo;
    }
// Set the device as configured - enables all endpoints
#[no_mangle]
pub unsafe extern "C" fn udc_set_device_configured(udc: *mut lpc32xx_udc) {
    static inline void udc_set_device_configured(struct lpc32xx_udc *udc)
    {
    udc_protocol_cmd_data_w(udc, CMD_CFG_DEV, DAT_WR_BYTE(CONF_DVICE));
    }
// Set the device as unconfigured - disables all endpoints
#[no_mangle]
pub unsafe extern "C" fn udc_set_device_unconfigured(udc: *mut lpc32xx_udc) {
    static inline void udc_set_device_unconfigured(struct lpc32xx_udc *udc)
    {
    udc_protocol_cmd_data_w(udc, CMD_CFG_DEV, DAT_WR_BYTE(0));
    }
// reinit == restore initial software state
#[no_mangle]
unsafe extern "C" fn udc_reinit(udc: *mut lpc32xx_udc) {
    static void udc_reinit(struct lpc32xx_udc *udc)
    {
    u32 i;
    INIT_LIST_HEAD(&udc.gadget.ep_list);
    INIT_LIST_HEAD(&udc.gadget.ep0.ep_list);
    for (i = 0; i < NUM_ENDPOINTS; i++) {
    struct lpc32xx_ep *ep = &udc.ep[i];
    if (i != 0)
    list_add_tail(&ep.ep.ep_list, &udc.gadget.ep_list);
    usb_ep_set_maxpacket_limit(&ep.ep, ep.maxpacket);
    INIT_LIST_HEAD(&ep.queue);
    ep.req_pending = 0;
    }
    udc.ep0state = WAIT_FOR_SETUP;
    }
// Must be called with lock
#[no_mangle]
unsafe extern "C" fn done(ep: *mut lpc32xx_ep, req: *mut lpc32xx_request, status: c_int) {
    static void done(struct lpc32xx_ep *ep, struct lpc32xx_request *req, int status)
    {
    struct lpc32xx_udc *udc = ep.udc;
    list_del_init(&req.queue);
    if (req.req.status == -EINPROGRESS)
    req.req.status = status;
    else
    status = req.req.status;
    if (ep.lep) {
    usb_gadget_unmap_request(&udc.gadget, &req.req, ep.is_in);
// Free DDs
    udc_dd_free(udc, req.dd_desc_ptr);
    }
    if (status && status != -ESHUTDOWN)
    ep_dbg(ep, "%s done %p, status %d\n", ep.ep.name, req, status);
    ep.req_pending = 0;
    spin_unlock(&udc.lock);
    usb_gadget_giveback_request(&ep.ep, &req.req);
    spin_lock(&udc.lock);
    }
// Must be called with lock
#[no_mangle]
unsafe extern "C" fn nuke(ep: *mut lpc32xx_ep, status: c_int) {
    static void nuke(struct lpc32xx_ep *ep, int status)
    {
    struct lpc32xx_request *req;
    while (!list_empty(&ep.queue)) {
    req = list_entry(ep.queue.next, struct lpc32xx_request, queue);
    done(ep, req, status);
    }
    if (status == -ESHUTDOWN) {
    uda_disable_hwepint(ep.udc, ep.hwep_num);
    udc_disable_hwep(ep.udc, ep.hwep_num);
    }
    }
// IN endpoint 0 transfer
#[no_mangle]
unsafe extern "C" fn udc_ep0_in_req(udc: *mut lpc32xx_udc) -> c_int {
    static int udc_ep0_in_req(struct lpc32xx_udc *udc)
    {
    struct lpc32xx_request *req;
    struct lpc32xx_ep *ep0 = &udc.ep[0];
    u32 tsend, ts = 0;
    if (list_empty(&ep0.queue))
// Nothing to send
    return 0;
    else
    req = list_entry(ep0.queue.next, struct lpc32xx_request,
    queue);
    tsend = ts = req.req.length - req.req.actual;
    if (ts == 0) {
// Send a ZLP
    udc_ep0_send_zlp(udc);
    done(ep0, req, 0);
    return 1;
    } else if (ts > ep0.ep.maxpacket)
    ts = ep0.ep.maxpacket; /* Just send what we can */
// Write data to the EP0 FIFO and start transfer
    udc_write_hwep(udc, EP_IN, (req.req.buf + req.req.actual), ts);
// Increment data pointer
    req.req.actual += ts;
    if (tsend >= ep0.ep.maxpacket)
    return 0; /* Stay in data transfer state */
// Transfer request is complete
    udc.ep0state = WAIT_FOR_SETUP;
    done(ep0, req, 0);
    return 1;
    }
// OUT endpoint 0 transfer
#[no_mangle]
unsafe extern "C" fn udc_ep0_out_req(udc: *mut lpc32xx_udc) -> c_int {
    static int udc_ep0_out_req(struct lpc32xx_udc *udc)
    {
    struct lpc32xx_request *req;
    struct lpc32xx_ep *ep0 = &udc.ep[0];
    u32 tr, bufferspace;
    if (list_empty(&ep0.queue))
    return 0;
    else
    req = list_entry(ep0.queue.next, struct lpc32xx_request,
    queue);
    if (req.req.length == 0) {
// Just dequeue request
    done(ep0, req, 0);
    udc.ep0state = WAIT_FOR_SETUP;
    return 1;
    }
// Get data from FIFO
    bufferspace = req.req.length - req.req.actual;
    if (bufferspace > ep0.ep.maxpacket)
    bufferspace = ep0.ep.maxpacket;
// Copy data to buffer
    prefetchw(req.req.buf + req.req.actual);
    tr = udc_read_hwep(udc, EP_OUT, req.req.buf + req.req.actual,
    bufferspace);
    req.req.actual += bufferspace;
    if (tr < ep0.ep.maxpacket) {
// This is the last packet
    done(ep0, req, 0);
    udc.ep0state = WAIT_FOR_SETUP;
    return 1;
    }
    return 0;
    }
// Must be called with lock
#[no_mangle]
unsafe extern "C" fn stop_activity(udc: *mut lpc32xx_udc) {
    static void stop_activity(struct lpc32xx_udc *udc)
    {
    struct usb_gadget_driver *driver = udc.driver;
    int i;
    if (udc.gadget.speed == USB_SPEED_UNKNOWN)
    driver = core::ptr::null_mut();
    udc.gadget.speed = USB_SPEED_UNKNOWN;
    udc.suspended = 0;
    for (i = 0; i < NUM_ENDPOINTS; i++) {
    struct lpc32xx_ep *ep = &udc.ep[i];
    nuke(ep, -ESHUTDOWN);
    }
    if (driver) {
    spin_unlock(&udc.lock);
    driver.disconnect(&udc.gadget);
    spin_lock(&udc.lock);
    }
    isp1301_pullup_enable(udc, 0, 0);
    udc_disable(udc);
    udc_reinit(udc);
    }
//
// Activate or kill host pullup
// Can be called with or without lock
//
#[no_mangle]
unsafe extern "C" fn pullup(udc: *mut lpc32xx_udc, is_on: c_int) {
    static void pullup(struct lpc32xx_udc *udc, int is_on)
    {
    if (!udc.clocked)
    return;
    if (!udc.enabled || !udc.vbus)
    is_on = 0;
    if (is_on != udc.pullup)
    isp1301_pullup_enable(udc, is_on, 0);
    }
// Must be called without lock
#[no_mangle]
unsafe extern "C" fn lpc32xx_ep_disable(_ep: *mut usb_ep) -> c_int {
    static int lpc32xx_ep_disable(struct usb_ep *_ep)
    {
    struct lpc32xx_ep *ep = container_of(_ep, struct lpc32xx_ep, ep);
    struct lpc32xx_udc *udc = ep.udc;
    unsigned long	flags;
    if ((ep.hwep_num_base == 0) || (ep.hwep_num == 0))
    return -EINVAL;
    spin_lock_irqsave(&udc.lock, flags);
    nuke(ep, -ESHUTDOWN);
// Clear all DMA statuses for this EP
    udc_ep_dma_disable(udc, ep.hwep_num);
    writel(1 << ep.hwep_num, USBD_EOTINTCLR(udc.udp_baseaddr));
    writel(1 << ep.hwep_num, USBD_NDDRTINTCLR(udc.udp_baseaddr));
    writel(1 << ep.hwep_num, USBD_SYSERRTINTCLR(udc.udp_baseaddr));
    writel(1 << ep.hwep_num, USBD_DMARCLR(udc.udp_baseaddr));
// Remove the DD pointer in the UDCA
    udc.udca_v_base[ep.hwep_num] = 0;
// Disable and reset endpoint and interrupt
    uda_clear_hwepint(udc, ep.hwep_num);
    udc_unrealize_hwep(udc, ep.hwep_num);
    ep.hwep_num = 0;
    spin_unlock_irqrestore(&udc.lock, flags);
    atomic_dec(&udc.enabled_ep_cnt);
    wake_up(&udc.ep_disable_wait_queue);
    return 0;
    }
// Must be called without lock
    static int lpc32xx_ep_enable(struct usb_ep *_ep,
    const struct usb_endpoint_descriptor *desc)
    {
    struct lpc32xx_ep *ep = container_of(_ep, struct lpc32xx_ep, ep);
    struct lpc32xx_udc *udc;
    u16 maxpacket;
    u32 tmp;
    unsigned long flags;
// Verify EP data
    if ((!_ep) || (!ep) || (!desc) ||
    (desc.bDescriptorType != USB_DT_ENDPOINT))
    return -EINVAL;
    udc = ep.udc;
    maxpacket = usb_endpoint_maxp(desc);
    if ((maxpacket == 0) || (maxpacket > ep.maxpacket)) {
    dev_dbg(udc.dev, "bad ep descriptor's packet size\n");
    return -EINVAL;
    }
// Don't touch EP0
    if (ep.hwep_num_base == 0) {
    dev_dbg(udc.dev, "Can't re-enable EP0!!!\n");
    return -EINVAL;
    }
// Is driver ready?
    if ((!udc.driver) || (udc.gadget.speed == USB_SPEED_UNKNOWN)) {
    dev_dbg(udc.dev, "bogus device state\n");
    return -ESHUTDOWN;
    }
    tmp = usb_endpoint_type(desc);
    switch (tmp) {
    case USB_ENDPOINT_XFER_CONTROL:
    return -EINVAL;
    case USB_ENDPOINT_XFER_INT:
    if (maxpacket > ep.maxpacket) {
    dev_dbg(udc.dev,
    "Bad INT endpoint maxpacket %d\n", maxpacket);
    return -EINVAL;
    }
    break;
    case USB_ENDPOINT_XFER_BULK:
    switch (maxpacket) {
    case 8:
    case 16:
    case 32:
    case 64:
    break;
    default:
    dev_dbg(udc.dev,
    "Bad BULK endpoint maxpacket %d\n", maxpacket);
    return -EINVAL;
    }
    break;
    case USB_ENDPOINT_XFER_ISOC:
    break;
    }
    spin_lock_irqsave(&udc.lock, flags);
// Initialize endpoint to match the selected descriptor
    ep.is_in = (desc.bEndpointAddress & USB_DIR_IN) != 0;
    ep.ep.maxpacket = maxpacket;
// Map hardware endpoint from base and direction
    if (ep.is_in)
// IN endpoints are offset 1 from the OUT endpoint
    ep.hwep_num = ep.hwep_num_base + EP_IN;
    else
    ep.hwep_num = ep.hwep_num_base;
    ep_dbg(ep, "EP enabled: %s, HW:%d, MP:%d IN:%d\n", ep.ep.name,
    ep.hwep_num, maxpacket, (ep.is_in == 1));
// Realize the endpoint, interrupt is enabled later when
// buffers are queued, IN EPs will NAK until buffers are ready
    udc_realize_hwep(udc, ep.hwep_num, ep.ep.maxpacket);
    udc_clr_buffer_hwep(udc, ep.hwep_num);
    uda_disable_hwepint(udc, ep.hwep_num);
    udc_clrstall_hwep(udc, ep.hwep_num);
// Clear all DMA statuses for this EP
    udc_ep_dma_disable(udc, ep.hwep_num);
    writel(1 << ep.hwep_num, USBD_EOTINTCLR(udc.udp_baseaddr));
    writel(1 << ep.hwep_num, USBD_NDDRTINTCLR(udc.udp_baseaddr));
    writel(1 << ep.hwep_num, USBD_SYSERRTINTCLR(udc.udp_baseaddr));
    writel(1 << ep.hwep_num, USBD_DMARCLR(udc.udp_baseaddr));
    spin_unlock_irqrestore(&udc.lock, flags);
    atomic_inc(&udc.enabled_ep_cnt);
    return 0;
    }
//
// Allocate a USB request list
// Can be called with or without lock
//
    static struct usb_request *lpc32xx_ep_alloc_request(struct usb_ep *_ep,
    gfp_t gfp_flags)
    {
    struct lpc32xx_request *req;
    req = kzalloc_obj(struct lpc32xx_request, gfp_flags);
    if (!req)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&req.queue);
    return &req.req;
    }
//
// De-allocate a USB request list
// Can be called with or without lock
//
    static void lpc32xx_ep_free_request(struct usb_ep *_ep,
    struct usb_request *_req)
    {
    struct lpc32xx_request *req;
    req = container_of(_req, struct lpc32xx_request, req);
    BUG_ON(!list_empty(&req.queue));
    kfree(req);
    }
// Must be called without lock
    static int lpc32xx_ep_queue(struct usb_ep *_ep,
    struct usb_request *_req, gfp_t gfp_flags)
    {
    struct lpc32xx_request *req;
    struct lpc32xx_ep *ep;
    struct lpc32xx_udc *udc;
    unsigned long flags;
    let mut status: c_int = 0;
    req = container_of(_req, struct lpc32xx_request, req);
    ep = container_of(_ep, struct lpc32xx_ep, ep);
    if (!_ep || !_req || !_req.complete || !_req.buf ||
    !list_empty(&req.queue))
    return -EINVAL;
    udc = ep.udc;
    if (udc.gadget.speed == USB_SPEED_UNKNOWN)
    return -EPIPE;
    if (ep.lep) {
    struct lpc32xx_usbd_dd_gad *dd;
    status = usb_gadget_map_request(&udc.gadget, _req, ep.is_in);
    if (status)
    return status;
// For the request, build a list of DDs
    dd = udc_dd_alloc(udc);
    if (!dd) {
// Error allocating DD
    return -ENOMEM;
    }
    req.dd_desc_ptr = dd;
// Setup the DMA descriptor
    dd.dd_next_phy = dd.dd_next_v = 0;
    dd.dd_buffer_addr = req.req.dma;
    dd.dd_status = 0;
// Special handling for ISO EPs
    if (ep.eptype == EP_ISO_TYPE) {
    dd.dd_setup = DD_SETUP_ISO_EP |
    DD_SETUP_PACKETLEN(0) |
    DD_SETUP_DMALENBYTES(1);
    dd.dd_iso_ps_mem_addr = dd.this_dma + 24;
    if (ep.is_in)
    dd.iso_status[0] = req.req.length;
    else
    dd.iso_status[0] = 0;
    } else
    dd.dd_setup = DD_SETUP_PACKETLEN(ep.ep.maxpacket) |
    DD_SETUP_DMALENBYTES(req.req.length);
    }
    ep_dbg(ep, "%s queue req %p len %d buf %p (in=%d) z=%d\n", _ep.name,
    _req, _req.length, _req.buf, ep.is_in, _req.zero);
    spin_lock_irqsave(&udc.lock, flags);
    _req.status = -EINPROGRESS;
    _req.actual = 0;
    req.send_zlp = _req.zero;
// Kickstart empty queues
    if (list_empty(&ep.queue)) {
    list_add_tail(&req.queue, &ep.queue);
    if (ep.hwep_num_base == 0) {
// Handle expected data direction
    if (ep.is_in) {
// IN packet to host
    udc.ep0state = DATA_IN;
    status = udc_ep0_in_req(udc);
    } else {
// OUT packet from host
    udc.ep0state = DATA_OUT;
    status = udc_ep0_out_req(udc);
    }
    } else if (ep.is_in) {
// IN packet to host and kick off transfer
    if (!ep.req_pending)
    udc_ep_in_req_dma(udc, ep);
    } else
// OUT packet from host and kick off list
    if (!ep.req_pending)
    udc_ep_out_req_dma(udc, ep);
    } else
    list_add_tail(&req.queue, &ep.queue);
    spin_unlock_irqrestore(&udc.lock, flags);
    return (status < 0) ? status : 0;
    }
// Must be called without lock
#[no_mangle]
unsafe extern "C" fn lpc32xx_ep_dequeue(_ep: *mut usb_ep, _req: *mut usb_request) -> c_int {
    static int lpc32xx_ep_dequeue(struct usb_ep *_ep, struct usb_request *_req)
    {
    struct lpc32xx_ep *ep;
    struct lpc32xx_request *req = core::ptr::null_mut(), *iter;
    unsigned long flags;
    ep = container_of(_ep, struct lpc32xx_ep, ep);
    if (!_ep || ep.hwep_num_base == 0)
    return -EINVAL;
    spin_lock_irqsave(&ep.udc.lock, flags);
// make sure it's actually queued on this endpoint
    list_for_each_entry(iter, &ep.queue, queue) {
    if (&iter.req != _req)
    continue;
    req = iter;
    break;
    }
    if (!req) {
    spin_unlock_irqrestore(&ep.udc.lock, flags);
    return -EINVAL;
    }
    done(ep, req, -ECONNRESET);
    spin_unlock_irqrestore(&ep.udc.lock, flags);
    return 0;
    }
// Must be called without lock
#[no_mangle]
unsafe extern "C" fn lpc32xx_ep_set_halt(_ep: *mut usb_ep, value: c_int) -> c_int {
    static int lpc32xx_ep_set_halt(struct usb_ep *_ep, int value)
    {
    struct lpc32xx_ep *ep = container_of(_ep, struct lpc32xx_ep, ep);
    struct lpc32xx_udc *udc;
    unsigned long flags;
    if ((!ep) || (ep.hwep_num <= 1))
    return -EINVAL;
// Don't halt an IN EP
    if (ep.is_in)
    return -EAGAIN;
    udc = ep.udc;
    spin_lock_irqsave(&udc.lock, flags);
    if (value == 1) {
// stall
    udc_protocol_cmd_data_w(udc, CMD_SET_EP_STAT(ep.hwep_num),
    DAT_WR_BYTE(EP_STAT_ST));
    } else {
// End stall
    ep.wedge = 0;
    udc_protocol_cmd_data_w(udc, CMD_SET_EP_STAT(ep.hwep_num),
    DAT_WR_BYTE(0));
    }
    spin_unlock_irqrestore(&udc.lock, flags);
    return 0;
    }
// set the halt feature and ignores clear requests
#[no_mangle]
unsafe extern "C" fn lpc32xx_ep_set_wedge(_ep: *mut usb_ep) -> c_int {
    static int lpc32xx_ep_set_wedge(struct usb_ep *_ep)
    {
    struct lpc32xx_ep *ep = container_of(_ep, struct lpc32xx_ep, ep);
    if (!_ep || !ep.udc)
    return -EINVAL;
    ep.wedge = 1;
    return usb_ep_set_halt(_ep);
    }
    static const struct usb_ep_ops lpc32xx_ep_ops = {
    .enable		= lpc32xx_ep_enable,
    .disable	= lpc32xx_ep_disable,
    .alloc_request	= lpc32xx_ep_alloc_request,
    .free_request	= lpc32xx_ep_free_request,
    .queue		= lpc32xx_ep_queue,
    .dequeue	= lpc32xx_ep_dequeue,
    .set_halt	= lpc32xx_ep_set_halt,
    .set_wedge	= lpc32xx_ep_set_wedge,
    };
// Send a ZLP on a non-0 IN EP
#[no_mangle]
unsafe extern "C" fn udc_send_in_zlp(udc: *mut lpc32xx_udc, ep: *mut lpc32xx_ep) {
    static void udc_send_in_zlp(struct lpc32xx_udc *udc, struct lpc32xx_ep *ep)
    {
// Clear EP status
    udc_clearep_getsts(udc, ep.hwep_num);
// Send ZLP via FIFO mechanism
    udc_write_hwep(udc, ep.hwep_num, core::ptr::null_mut(), 0);
    }
//
// Handle EP completion for ZLP
// This function will only be called when a delayed ZLP needs to be sent out
// after a DMA transfer has filled both buffers.
//
#[no_mangle]
unsafe extern "C" fn udc_handle_eps(udc: *mut lpc32xx_udc, ep: *mut lpc32xx_ep) {
    static void udc_handle_eps(struct lpc32xx_udc *udc, struct lpc32xx_ep *ep)
    {
    u32 epstatus;
    struct lpc32xx_request *req;
    if (ep.hwep_num <= 0)
    return;
    uda_clear_hwepint(udc, ep.hwep_num);
// If this interrupt isn't enabled, return now
    if (!(udc.enabled_hwepints & (1 << ep.hwep_num)))
    return;
// Get endpoint status
    epstatus = udc_clearep_getsts(udc, ep.hwep_num);
//
// This should never happen, but protect against writing to the
// buffer when full.
//
    if (epstatus & EP_SEL_F)
    return;
    if (ep.is_in) {
    udc_send_in_zlp(udc, ep);
    uda_disable_hwepint(udc, ep.hwep_num);
    } else
    return;
// If there isn't a request waiting, something went wrong
    req = list_entry(ep.queue.next, struct lpc32xx_request, queue);
    done(ep, req, 0);
// Start another request if ready
    if (!list_empty(&ep.queue)) {
    if (ep.is_in)
    udc_ep_in_req_dma(udc, ep);
    else
    udc_ep_out_req_dma(udc, ep);
    } else
    ep.req_pending = 0;
    }
// DMA end of transfer completion
#[no_mangle]
unsafe extern "C" fn udc_handle_dma_ep(udc: *mut lpc32xx_udc, ep: *mut lpc32xx_ep) {
    static void udc_handle_dma_ep(struct lpc32xx_udc *udc, struct lpc32xx_ep *ep)
    {
    u32 status;
    struct lpc32xx_request *req;
    struct lpc32xx_usbd_dd_gad *dd;

    ep.totalints++;

    req = list_entry(ep.queue.next, struct lpc32xx_request, queue);
    dd = req.dd_desc_ptr;
// DMA descriptor should always be retired for this call
    if (!(dd.dd_status & DD_STATUS_DD_RETIRED))
    ep_warn(ep, "DMA descriptor did not retire\n");
// Disable DMA
    udc_ep_dma_disable(udc, ep.hwep_num);
    writel((1 << ep.hwep_num), USBD_EOTINTCLR(udc.udp_baseaddr));
    writel((1 << ep.hwep_num), USBD_NDDRTINTCLR(udc.udp_baseaddr));
// System error?
    if (readl(USBD_SYSERRTINTST(udc.udp_baseaddr)) &
    (1 << ep.hwep_num)) {
    writel((1 << ep.hwep_num),
    USBD_SYSERRTINTCLR(udc.udp_baseaddr));
    ep_err(ep, "AHB critical error!\n");
    ep.req_pending = 0;
// The error could have occurred on a packet of a multipacket
// transfer, so recovering the transfer is not possible. Close
// the request with an error
    done(ep, req, -ECONNABORTED);
    return;
    }
// Handle the current DD's status
    status = dd.dd_status;
    switch (status & DD_STATUS_STS_MASK) {
    case DD_STATUS_STS_NS:
// DD not serviced? This shouldn't happen!
    ep.req_pending = 0;
    ep_err(ep, "DMA critical EP error: DD not serviced (0x%x)!\n",
    status);
    done(ep, req, -ECONNABORTED);
    return;
    case DD_STATUS_STS_BS:
// Interrupt only fires on EOT - This shouldn't happen!
    ep.req_pending = 0;
    ep_err(ep, "DMA critical EP error: EOT prior to service completion (0x%x)!\n",
    status);
    done(ep, req, -ECONNABORTED);
    return;
    case DD_STATUS_STS_NC:
    case DD_STATUS_STS_DUR:
// Really just a short packet, not an underrun
// This is a good status and what we expect
    break;
    default:
// Data overrun, system error, or unknown
    ep.req_pending = 0;
    ep_err(ep, "DMA critical EP error: System error (0x%x)!\n",
    status);
    done(ep, req, -ECONNABORTED);
    return;
    }
// ISO endpoints are handled differently
    if (ep.eptype == EP_ISO_TYPE) {
    if (ep.is_in)
    req.req.actual = req.req.length;
    else
    req.req.actual = dd.iso_status[0] & 0xFFFF;
    } else
    req.req.actual += DD_STATUS_CURDMACNT(status);
// Send a ZLP if necessary. This will be done for non-int
// packets which have a size that is a divisor of MAXP
    if (req.send_zlp) {
//
// If at least 1 buffer is available, send the ZLP now.
// Otherwise, the ZLP send needs to be deferred until a
// buffer is available.
//
    if (udc_clearep_getsts(udc, ep.hwep_num) & EP_SEL_F) {
    udc_clearep_getsts(udc, ep.hwep_num);
    uda_enable_hwepint(udc, ep.hwep_num);
    udc_clearep_getsts(udc, ep.hwep_num);
// Let the EP interrupt handle the ZLP
    return;
    } else
    udc_send_in_zlp(udc, ep);
    }
// Transfer request is complete
    done(ep, req, 0);
// Start another request if ready
    udc_clearep_getsts(udc, ep.hwep_num);
    if (!list_empty((&ep.queue))) {
    if (ep.is_in)
    udc_ep_in_req_dma(udc, ep);
    else
    udc_ep_out_req_dma(udc, ep);
    } else
    ep.req_pending = 0;
    }
//
// Endpoint 0 functions
//
#[no_mangle]
unsafe extern "C" fn udc_handle_dev(udc: *mut lpc32xx_udc) {
    static void udc_handle_dev(struct lpc32xx_udc *udc)
    {
    u32 tmp;
    udc_protocol_cmd_w(udc, CMD_GET_DEV_STAT);
    tmp = udc_protocol_cmd_r(udc, DAT_GET_DEV_STAT);
    if (tmp & DEV_RST)
    uda_usb_reset(udc);
#[no_mangle]
pub unsafe extern "C" fn if(DEV_CON_CH: tmp &) -> else {
    else if (tmp & DEV_CON_CH)
    uda_power_event(udc, (tmp & DEV_CON));
#[no_mangle]
pub unsafe extern "C" fn if(DEV_SUS_CH: tmp &) -> else {
    if (tmp & DEV_SUS) {
    if (udc.vbus == 0)
    stop_activity(udc);
    else if ((udc.gadget.speed != USB_SPEED_UNKNOWN) &&
    udc.driver) {
// Power down transceiver
    udc.poweron = 0;
    schedule_work(&udc.pullup_job);
    uda_resm_susp_event(udc, 1);
    }
    } else if ((udc.gadget.speed != USB_SPEED_UNKNOWN) &&
    udc.driver && udc.vbus) {
    uda_resm_susp_event(udc, 0);
// Power up transceiver
    udc.poweron = 1;
    schedule_work(&udc.pullup_job);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn udc_get_status(udc: *mut lpc32xx_udc, reqtype: u16, wIndex: u16) -> c_int {
    static int udc_get_status(struct lpc32xx_udc *udc, u16 reqtype, u16 wIndex)
    {
    struct lpc32xx_ep *ep;
    let mut ep0buff: u32 = 0, tmp;
    switch (reqtype & USB_RECIP_MASK) {
    case USB_RECIP_INTERFACE:
    break; /* Not supported */
    case USB_RECIP_DEVICE:
    ep0buff = udc.gadget.is_selfpowered;
    if (udc.dev_status & (1 << USB_DEVICE_REMOTE_WAKEUP))
    ep0buff |= (1 << USB_DEVICE_REMOTE_WAKEUP);
    break;
    case USB_RECIP_ENDPOINT:
    tmp = wIndex & USB_ENDPOINT_NUMBER_MASK;
    ep = &udc.ep[tmp];
    if ((tmp == 0) || (tmp >= NUM_ENDPOINTS))
    return -EOPNOTSUPP;
    if (wIndex & USB_DIR_IN) {
    if (!ep.is_in)
    return -EOPNOTSUPP; /* Something's wrong */
    } else if (ep.is_in)
    return -EOPNOTSUPP; /* Not an IN endpoint */
// Get status of the endpoint
    udc_protocol_cmd_w(udc, CMD_SEL_EP(ep.hwep_num));
    tmp = udc_protocol_cmd_r(udc, DAT_SEL_EP(ep.hwep_num));
    if (tmp & EP_SEL_ST)
    ep0buff = (1 << USB_ENDPOINT_HALT);
    else
    ep0buff = 0;
    break;
    default:
    break;
    }
// Return data
    udc_write_hwep(udc, EP_IN, &ep0buff, 2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udc_handle_ep0_setup(udc: *mut lpc32xx_udc) {
    static void udc_handle_ep0_setup(struct lpc32xx_udc *udc)
    {
    struct lpc32xx_ep *ep, *ep0 = &udc.ep[0];
    struct usb_ctrlrequest ctrlpkt;
    int i, bytes;
    u16 wIndex, wValue, reqtype, req, tmp;
// Nuke previous transfers
    nuke(ep0, -EPROTO);
// Get setup packet
    bytes = udc_read_hwep(udc, EP_OUT, (u32 *) &ctrlpkt, 8);
    if (bytes != 8) {
    ep_warn(ep0, "Incorrectly sized setup packet (s/b 8, is %d)!\n",
    bytes);
    return;
    }
// Native endianness
    wIndex = le16_to_cpu(ctrlpkt.wIndex);
    wValue = le16_to_cpu(ctrlpkt.wValue);
    reqtype = le16_to_cpu(ctrlpkt.bRequestType);
// Set direction of EP0
    if (likely(reqtype & USB_DIR_IN))
    ep0.is_in = 1;
    else
    ep0.is_in = 0;
// Handle SETUP packet
    req = le16_to_cpu(ctrlpkt.bRequest);
    switch (req) {
    case USB_REQ_CLEAR_FEATURE:
    case USB_REQ_SET_FEATURE:
    switch (reqtype) {
    case (USB_TYPE_STANDARD | USB_RECIP_DEVICE):
    if (wValue != USB_DEVICE_REMOTE_WAKEUP)
    goto stall; /* Nothing else handled */
// Tell board about event
    if (req == USB_REQ_CLEAR_FEATURE)
    udc.dev_status &=
    ~(1 << USB_DEVICE_REMOTE_WAKEUP);
    else
    udc.dev_status |=
    (1 << USB_DEVICE_REMOTE_WAKEUP);
    uda_remwkp_cgh(udc);
    goto zlp_send;
    case (USB_TYPE_STANDARD | USB_RECIP_ENDPOINT):
    tmp = wIndex & USB_ENDPOINT_NUMBER_MASK;
    if ((wValue != USB_ENDPOINT_HALT) ||
    (tmp >= NUM_ENDPOINTS))
    break;
// Find hardware endpoint from logical endpoint
    ep = &udc.ep[tmp];
    tmp = ep.hwep_num;
    if (tmp == 0)
    break;
    if (req == USB_REQ_SET_FEATURE)
    udc_stall_hwep(udc, tmp);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !ep->wedge) -> else {
    else if (!ep.wedge)
    udc_clrstall_hwep(udc, tmp);
    goto zlp_send;
    default:
    break;
    }
    break;
    case USB_REQ_SET_ADDRESS:
    if (reqtype == (USB_TYPE_STANDARD | USB_RECIP_DEVICE)) {
    udc_set_address(udc, wValue);
    goto zlp_send;
    }
    break;
    case USB_REQ_GET_STATUS:
    udc_get_status(udc, reqtype, wIndex);
    return;
    default:
    break; /* Let GadgetFS handle the descriptor instead */
    }
    if (likely(udc.driver)) {
// device-2-host (IN) or no data setup command, process
// immediately
    spin_unlock(&udc.lock);
    i = udc.driver.setup(&udc.gadget, &ctrlpkt);
    spin_lock(&udc.lock);
    if (req == USB_REQ_SET_CONFIGURATION) {
// Configuration is set after endpoints are realized
    if (wValue) {
// Set configuration
    udc_set_device_configured(udc);
    udc_protocol_cmd_data_w(udc, CMD_SET_MODE,
    DAT_WR_BYTE(AP_CLK |
    INAK_BI | INAK_II));
    } else {
// Clear configuration
    udc_set_device_unconfigured(udc);
// Disable NAK interrupts
    udc_protocol_cmd_data_w(udc, CMD_SET_MODE,
    DAT_WR_BYTE(AP_CLK));
    }
    }
    if (i < 0) {
// setup processing failed, force stall
    dev_dbg(udc.dev,
    "req %02x.%02x protocol STALL; stat %d\n",
    reqtype, req, i);
    udc.ep0state = WAIT_FOR_SETUP;
    goto stall;
    }
    }
    if (!ep0.is_in)
    udc_ep0_send_zlp(udc); /* ZLP IN packet on data phase */
    return;
    stall:
    udc_stall_hwep(udc, EP_IN);
    return;
    zlp_send:
    udc_ep0_send_zlp(udc);
    return;
    }
// IN endpoint 0 transfer
#[no_mangle]
unsafe extern "C" fn udc_handle_ep0_in(udc: *mut lpc32xx_udc) {
    static void udc_handle_ep0_in(struct lpc32xx_udc *udc)
    {
    struct lpc32xx_ep *ep0 = &udc.ep[0];
    u32 epstatus;
// Clear EP interrupt
    epstatus = udc_clearep_getsts(udc, EP_IN);

    ep0.totalints++;

// Stalled? Clear stall and reset buffers
    if (epstatus & EP_SEL_ST) {
    udc_clrstall_hwep(udc, EP_IN);
    nuke(ep0, -ECONNABORTED);
    udc.ep0state = WAIT_FOR_SETUP;
    return;
    }
// Is a buffer available?
    if (!(epstatus & EP_SEL_F)) {
// Handle based on current state
    if (udc.ep0state == DATA_IN)
    udc_ep0_in_req(udc);
    else {
// Unknown state for EP0 oe end of DATA IN phase
    nuke(ep0, -ECONNABORTED);
    udc.ep0state = WAIT_FOR_SETUP;
    }
    }
    }
// OUT endpoint 0 transfer
#[no_mangle]
unsafe extern "C" fn udc_handle_ep0_out(udc: *mut lpc32xx_udc) {
    static void udc_handle_ep0_out(struct lpc32xx_udc *udc)
    {
    struct lpc32xx_ep *ep0 = &udc.ep[0];
    u32 epstatus;
// Clear EP interrupt
    epstatus = udc_clearep_getsts(udc, EP_OUT);

    ep0.totalints++;

// Stalled?
    if (epstatus & EP_SEL_ST) {
    udc_clrstall_hwep(udc, EP_OUT);
    nuke(ep0, -ECONNABORTED);
    udc.ep0state = WAIT_FOR_SETUP;
    return;
    }
// A NAK may occur if a packet couldn't be received yet
    if (epstatus & EP_SEL_EPN)
    return;
// Setup packet incoming?
    if (epstatus & EP_SEL_STP) {
    nuke(ep0, 0);
    udc.ep0state = WAIT_FOR_SETUP;
    }
// Data available?
    if (epstatus & EP_SEL_F)
// Handle based on current state
    switch (udc.ep0state) {
    case WAIT_FOR_SETUP:
    udc_handle_ep0_setup(udc);
    break;
    case DATA_OUT:
    udc_ep0_out_req(udc);
    break;
    default:
// Unknown state for EP0
    nuke(ep0, -ECONNABORTED);
    udc.ep0state = WAIT_FOR_SETUP;
    }
    }
// Must be called without lock
#[no_mangle]
unsafe extern "C" fn lpc32xx_get_frame(gadget: *mut usb_gadget) -> c_int {
    static int lpc32xx_get_frame(struct usb_gadget *gadget)
    {
    int frame;
    unsigned long flags;
    struct lpc32xx_udc *udc = to_udc(gadget);
    if (!udc.clocked)
    return -EINVAL;
    spin_lock_irqsave(&udc.lock, flags);
    frame = (int) udc_get_current_frame(udc);
    spin_unlock_irqrestore(&udc.lock, flags);
    return frame;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_wakeup(gadget: *mut usb_gadget) -> c_int {
    static int lpc32xx_wakeup(struct usb_gadget *gadget)
    {
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_set_selfpowered(gadget: *mut usb_gadget, is_on: c_int) -> c_int {
    static int lpc32xx_set_selfpowered(struct usb_gadget *gadget, int is_on)
    {
    gadget.is_selfpowered = (is_on != 0);
    return 0;
    }
//
// vbus is here!  turn everything on that's ready
// Must be called without lock
//
#[no_mangle]
unsafe extern "C" fn lpc32xx_vbus_session(gadget: *mut usb_gadget, is_active: c_int) -> c_int {
    static int lpc32xx_vbus_session(struct usb_gadget *gadget, int is_active)
    {
    unsigned long flags;
    struct lpc32xx_udc *udc = to_udc(gadget);
    spin_lock_irqsave(&udc.lock, flags);
// Doesn't need lock
    if (udc.driver) {
    udc_clk_set(udc, 1);
    udc_enable(udc);
    pullup(udc, is_active);
    } else {
    stop_activity(udc);
    pullup(udc, 0);
    spin_unlock_irqrestore(&udc.lock, flags);
//
// Wait for all the endpoints to disable,
// before disabling clocks. Don't wait if
// endpoints are not enabled.
//
    if (atomic_read(&udc.enabled_ep_cnt))
    wait_event_interruptible(udc.ep_disable_wait_queue,
    (atomic_read(&udc.enabled_ep_cnt) == 0));
    spin_lock_irqsave(&udc.lock, flags);
    udc_clk_set(udc, 0);
    }
    spin_unlock_irqrestore(&udc.lock, flags);
    return 0;
    }
// Can be called with or without lock
#[no_mangle]
unsafe extern "C" fn lpc32xx_pullup(gadget: *mut usb_gadget, is_on: c_int) -> c_int {
    static int lpc32xx_pullup(struct usb_gadget *gadget, int is_on)
    {
    struct lpc32xx_udc *udc = to_udc(gadget);
// Doesn't need lock
    pullup(udc, is_on);
    return 0;
    }
    static int lpc32xx_start(struct usb_gadget *, struct usb_gadget_driver *);
    static int lpc32xx_stop(struct usb_gadget *);
    static const struct usb_gadget_ops lpc32xx_udc_ops = {
    .get_frame		= lpc32xx_get_frame,
    .wakeup			= lpc32xx_wakeup,
    .set_selfpowered	= lpc32xx_set_selfpowered,
    .vbus_session		= lpc32xx_vbus_session,
    .pullup			= lpc32xx_pullup,
    .udc_start		= lpc32xx_start,
    .udc_stop		= lpc32xx_stop,
    };
#[no_mangle]
unsafe extern "C" fn nop_release(dev: *mut device) {
    static void nop_release(struct device *dev)
    {
// nothing to free
    }
    static const struct lpc32xx_udc controller_template = {
    .gadget = {
    .ops	= &lpc32xx_udc_ops,
    .name	= driver_name,
    .dev	= {
    .init_name = "gadget",
    .release = nop_release,
    }
    },
    .ep[0] = {
    .ep = {
    .name	= "ep0",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_CONTROL,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 0,
    .hwep_num	= 0, /* Can be 0 or 1, has special handling */
    .lep		= 0,
    .eptype		= EP_CTL_TYPE,
    },
    .ep[1] = {
    .ep = {
    .name	= "ep1-int",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_INT,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 2,
    .hwep_num	= 0, /* 2 or 3, will be set later */
    .lep		= 1,
    .eptype		= EP_INT_TYPE,
    },
    .ep[2] = {
    .ep = {
    .name	= "ep2-bulk",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_BULK,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 4,
    .hwep_num	= 0, /* 4 or 5, will be set later */
    .lep		= 2,
    .eptype		= EP_BLK_TYPE,
    },
    .ep[3] = {
    .ep = {
    .name	= "ep3-iso",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_ISO,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 1023,
    .hwep_num_base	= 6,
    .hwep_num	= 0, /* 6 or 7, will be set later */
    .lep		= 3,
    .eptype		= EP_ISO_TYPE,
    },
    .ep[4] = {
    .ep = {
    .name	= "ep4-int",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_INT,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 8,
    .hwep_num	= 0, /* 8 or 9, will be set later */
    .lep		= 4,
    .eptype		= EP_INT_TYPE,
    },
    .ep[5] = {
    .ep = {
    .name	= "ep5-bulk",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_BULK,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 10,
    .hwep_num	= 0, /* 10 or 11, will be set later */
    .lep		= 5,
    .eptype		= EP_BLK_TYPE,
    },
    .ep[6] = {
    .ep = {
    .name	= "ep6-iso",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_ISO,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 1023,
    .hwep_num_base	= 12,
    .hwep_num	= 0, /* 12 or 13, will be set later */
    .lep		= 6,
    .eptype		= EP_ISO_TYPE,
    },
    .ep[7] = {
    .ep = {
    .name	= "ep7-int",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_INT,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 14,
    .hwep_num	= 0,
    .lep		= 7,
    .eptype		= EP_INT_TYPE,
    },
    .ep[8] = {
    .ep = {
    .name	= "ep8-bulk",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_BULK,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 16,
    .hwep_num	= 0,
    .lep		= 8,
    .eptype		= EP_BLK_TYPE,
    },
    .ep[9] = {
    .ep = {
    .name	= "ep9-iso",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_ISO,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 1023,
    .hwep_num_base	= 18,
    .hwep_num	= 0,
    .lep		= 9,
    .eptype		= EP_ISO_TYPE,
    },
    .ep[10] = {
    .ep = {
    .name	= "ep10-int",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_INT,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 20,
    .hwep_num	= 0,
    .lep		= 10,
    .eptype		= EP_INT_TYPE,
    },
    .ep[11] = {
    .ep = {
    .name	= "ep11-bulk",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_BULK,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 22,
    .hwep_num	= 0,
    .lep		= 11,
    .eptype		= EP_BLK_TYPE,
    },
    .ep[12] = {
    .ep = {
    .name	= "ep12-iso",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_ISO,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 1023,
    .hwep_num_base	= 24,
    .hwep_num	= 0,
    .lep		= 12,
    .eptype		= EP_ISO_TYPE,
    },
    .ep[13] = {
    .ep = {
    .name	= "ep13-int",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_INT,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 26,
    .hwep_num	= 0,
    .lep		= 13,
    .eptype		= EP_INT_TYPE,
    },
    .ep[14] = {
    .ep = {
    .name	= "ep14-bulk",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_BULK,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 64,
    .hwep_num_base	= 28,
    .hwep_num	= 0,
    .lep		= 14,
    .eptype		= EP_BLK_TYPE,
    },
    .ep[15] = {
    .ep = {
    .name	= "ep15-bulk",
    .ops	= &lpc32xx_ep_ops,
    .caps	= USB_EP_CAPS(USB_EP_CAPS_TYPE_BULK,
    USB_EP_CAPS_DIR_ALL),
    },
    .maxpacket	= 1023,
    .hwep_num_base	= 30,
    .hwep_num	= 0,
    .lep		= 15,
    .eptype		= EP_BLK_TYPE,
    },
    };
// ISO and status interrupts
#[no_mangle]
unsafe extern "C" fn lpc32xx_usb_lp_irq(irq: c_int, _udc: *mut c_void) -> irqreturn_t {
    static irqreturn_t lpc32xx_usb_lp_irq(int irq, void *_udc)
    {
    u32 tmp, devstat;
    struct lpc32xx_udc *udc = _udc;
    spin_lock(&udc.lock);
// Read the device status register
    devstat = readl(USBD_DEVINTST(udc.udp_baseaddr));
    devstat &= ~USBD_EP_FAST;
    writel(devstat, USBD_DEVINTCLR(udc.udp_baseaddr));
    devstat = devstat & udc.enabled_devints;
// Device specific handling needed?
    if (devstat & USBD_DEV_STAT)
    udc_handle_dev(udc);
// Start of frame? (devstat & FRAME_INT):
// The frame interrupt isn't really needed for ISO support,
// as the driver will queue the necessary packets
// Error?
    if (devstat & ERR_INT) {
// All types of errors, from cable removal during transfer to
// misc protocol and bit errors. These are mostly for just info,
// as the USB hardware will work around these. If these errors
// happen alot, something is wrong.
    udc_protocol_cmd_w(udc, CMD_RD_ERR_STAT);
    tmp = udc_protocol_cmd_r(udc, DAT_RD_ERR_STAT);
    dev_dbg(udc.dev, "Device error (0x%x)!\n", tmp);
    }
    spin_unlock(&udc.lock);
    return IRQ_HANDLED;
    }
// EP interrupts
#[no_mangle]
unsafe extern "C" fn lpc32xx_usb_hp_irq(irq: c_int, _udc: *mut c_void) -> irqreturn_t {
    static irqreturn_t lpc32xx_usb_hp_irq(int irq, void *_udc)
    {
    u32 tmp;
    struct lpc32xx_udc *udc = _udc;
    spin_lock(&udc.lock);
// Read the device status register
    writel(USBD_EP_FAST, USBD_DEVINTCLR(udc.udp_baseaddr));
// Endpoints
    tmp = readl(USBD_EPINTST(udc.udp_baseaddr));
// Special handling for EP0
    if (tmp & (EP_MASK_SEL(0, EP_OUT) | EP_MASK_SEL(0, EP_IN))) {
// Handle EP0 IN
    if (tmp & (EP_MASK_SEL(0, EP_IN)))
    udc_handle_ep0_in(udc);
// Handle EP0 OUT
    if (tmp & (EP_MASK_SEL(0, EP_OUT)))
    udc_handle_ep0_out(udc);
    }
// All other EPs
    if (tmp & ~(EP_MASK_SEL(0, EP_OUT) | EP_MASK_SEL(0, EP_IN))) {
    int i;
// Handle other EP interrupts
    for (i = 1; i < NUM_ENDPOINTS; i++) {
    if (tmp & (1 << udc.ep[i].hwep_num))
    udc_handle_eps(udc, &udc.ep[i]);
    }
    }
    spin_unlock(&udc.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_usb_devdma_irq(irq: c_int, _udc: *mut c_void) -> irqreturn_t {
    static irqreturn_t lpc32xx_usb_devdma_irq(int irq, void *_udc)
    {
    struct lpc32xx_udc *udc = _udc;
    int i;
    u32 tmp;
    spin_lock(&udc.lock);
// Handle EP DMA EOT interrupts
    tmp = readl(USBD_EOTINTST(udc.udp_baseaddr)) |
    (readl(USBD_EPDMAST(udc.udp_baseaddr)) &
    readl(USBD_NDDRTINTST(udc.udp_baseaddr))) |
    readl(USBD_SYSERRTINTST(udc.udp_baseaddr));
    for (i = 1; i < NUM_ENDPOINTS; i++) {
    if (tmp & (1 << udc.ep[i].hwep_num))
    udc_handle_dma_ep(udc, &udc.ep[i]);
    }
    spin_unlock(&udc.lock);
    return IRQ_HANDLED;
    }
//
// VBUS detection, pullup handler, and Gadget cable state notification
//
#[no_mangle]
unsafe extern "C" fn vbus_work(udc: *mut lpc32xx_udc) {
    static void vbus_work(struct lpc32xx_udc *udc)
    {
    u8 value;
    if (udc.enabled != 0) {
// Discharge VBUS real quick
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1, OTG1_VBUS_DISCHRG);
// Give VBUS some time (100mS) to discharge
    msleep(100);
// Disable VBUS discharge resistor
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR,
    OTG1_VBUS_DISCHRG);
// Clear interrupt
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_LATCH |
    ISP1301_I2C_REG_CLEAR_ADDR, ~0);
// Get the VBUS status from the transceiver
    value = i2c_smbus_read_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_SOURCE);
// VBUS on or off?
    if (value & INT_SESS_VLD)
    udc.vbus = 1;
    else
    udc.vbus = 0;
// VBUS changed?
    if (udc.last_vbus != udc.vbus) {
    udc.last_vbus = udc.vbus;
    lpc32xx_vbus_session(&udc.gadget, udc.vbus);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_usb_vbus_irq(irq: c_int, _udc: *mut c_void) -> irqreturn_t {
    static irqreturn_t lpc32xx_usb_vbus_irq(int irq, void *_udc)
    {
    struct lpc32xx_udc *udc = _udc;
    vbus_work(udc);
    return IRQ_HANDLED;
    }
    static int lpc32xx_start(struct usb_gadget *gadget,
    struct usb_gadget_driver *driver)
    {
    struct lpc32xx_udc *udc = to_udc(gadget);
    if (!driver || driver.max_speed < USB_SPEED_FULL || !driver.setup) {
    dev_err(udc.dev, "bad parameter.\n");
    return -EINVAL;
    }
    if (udc.driver) {
    dev_err(udc.dev, "UDC already has a gadget driver\n");
    return -EBUSY;
    }
    udc.driver = driver;
    udc.gadget.dev.of_node = udc.dev.of_node;
    udc.enabled = 1;
    udc.gadget.is_selfpowered = 1;
    udc.vbus = 0;
// Force VBUS process once to check for cable insertion
    udc.last_vbus = udc.vbus = 0;
    vbus_work(udc);
// enable interrupts
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_FALLING, INT_SESS_VLD | INT_VBUS_VLD);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_RISING, INT_SESS_VLD | INT_VBUS_VLD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_stop(gadget: *mut usb_gadget) -> c_int {
    static int lpc32xx_stop(struct usb_gadget *gadget)
    {
    struct lpc32xx_udc *udc = to_udc(gadget);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_FALLING | ISP1301_I2C_REG_CLEAR_ADDR, ~0);
    i2c_smbus_write_byte_data(udc.isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_RISING | ISP1301_I2C_REG_CLEAR_ADDR, ~0);
    if (udc.clocked) {
    spin_lock(&udc.lock);
    stop_activity(udc);
    spin_unlock(&udc.lock);
//
// Wait for all the endpoints to disable,
// before disabling clocks. Don't wait if
// endpoints are not enabled.
//
    if (atomic_read(&udc.enabled_ep_cnt))
    wait_event_interruptible(udc.ep_disable_wait_queue,
    (atomic_read(&udc.enabled_ep_cnt) == 0));
    spin_lock(&udc.lock);
    udc_clk_set(udc, 0);
    spin_unlock(&udc.lock);
    }
    udc.enabled = 0;
    udc.driver = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_udc_shutdown(dev: *mut platform_device) {
    static void lpc32xx_udc_shutdown(struct platform_device *dev)
    {
// Force disconnect on reboot
    struct lpc32xx_udc *udc = platform_get_drvdata(dev);
    pullup(udc, 0);
    }
//
// Callbacks to be overridden by options passed via OF (TODO)
//
#[no_mangle]
unsafe extern "C" fn lpc32xx_usbd_conn_chg(conn: c_int) {
    static void lpc32xx_usbd_conn_chg(int conn)
    {
// Do nothing, it might be nice to enable an LED
// based on conn state being !0
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_usbd_susp_chg(susp: c_int) {
    static void lpc32xx_usbd_susp_chg(int susp)
    {
// Device suspend if susp != 0
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_rmwkup_chg(remote_wakup_enable: c_int) {
    static void lpc32xx_rmwkup_chg(int remote_wakup_enable)
    {
// Enable or disable USB remote wakeup
    }
    static struct lpc32xx_usbd_cfg lpc32xx_usbddata = {
    .vbus_drv_pol = 0,
    .conn_chgb = &lpc32xx_usbd_conn_chg,
    .susp_chgb = &lpc32xx_usbd_susp_chg,
    .rmwk_chgb = &lpc32xx_rmwkup_chg,
    };
    let mut lpc32xx_usbd_dmamask: static u64 = ~(u32) 0x7F;
#[no_mangle]
unsafe extern "C" fn lpc32xx_udc_probe(pdev: *mut platform_device) -> c_int {
    static int lpc32xx_udc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct lpc32xx_udc *udc;
    int retval, i;
    dma_addr_t dma_handle;
    struct device_node *isp1301_node;
    udc = devm_kmemdup(dev, &controller_template, sizeof(*udc), GFP_KERNEL);
    if (!udc)
    return -ENOMEM;
    for (i = 0; i <= 15; i++)
    udc.ep[i].udc = udc;
    udc.gadget.ep0 = &udc.ep[0].ep;
// init software state
    udc.gadget.dev.parent = dev;
    udc.pdev = pdev;
    udc.dev = &pdev.dev;
    udc.enabled = 0;
    if (pdev.dev.of_node) {
    isp1301_node = of_parse_phandle(pdev.dev.of_node,
    "transceiver", 0);
    } else {
    isp1301_node = core::ptr::null_mut();
    }
    udc.isp1301_i2c_client = isp1301_get_client(isp1301_node);
    of_node_put(isp1301_node);
    if (!udc.isp1301_i2c_client) {
    return -EPROBE_DEFER;
    }
    dev_info(udc.dev, "ISP1301 I2C device at address 0x%x\n",
    udc.isp1301_i2c_client.addr);
    pdev.dev.dma_mask = &lpc32xx_usbd_dmamask;
    retval = dma_set_coherent_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (retval)
    goto err_put_client;
    udc.board = &lpc32xx_usbddata;
//
// Resources are mapped as follows:
// IORESOURCE_MEM, base address and size of USB space
// IORESOURCE_IRQ, USB device low priority interrupt number
// IORESOURCE_IRQ, USB device high priority interrupt number
// IORESOURCE_IRQ, USB device interrupt number
// IORESOURCE_IRQ, USB transceiver interrupt number
//
    spin_lock_init(&udc.lock);
// Get IRQs
    for (i = 0; i < 4; i++) {
    udc.udp_irq[i] = platform_get_irq(pdev, i);
    if (udc.udp_irq[i] < 0) {
    retval = udc.udp_irq[i];
    goto err_put_client;
    }
    }
    udc.udp_baseaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(udc.udp_baseaddr)) {
    dev_err(udc.dev, "IO map failure\n");
    retval = PTR_ERR(udc.udp_baseaddr);
    goto err_put_client;
    }
// Get USB device clock
    udc.usb_slv_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(udc.usb_slv_clk)) {
    dev_err(udc.dev, "failed to acquire USB device clock\n");
    retval = PTR_ERR(udc.usb_slv_clk);
    goto err_put_client;
    }
// Enable USB device clock
    retval = clk_prepare_enable(udc.usb_slv_clk);
    if (retval < 0) {
    dev_err(udc.dev, "failed to start USB device clock\n");
    goto err_put_client;
    }
// Setup deferred workqueue data
    udc.poweron = udc.pullup = 0;
    INIT_WORK(&udc.pullup_job, pullup_work);

    INIT_WORK(&udc.power_job, power_work);

// All clocks are now on
    udc.clocked = 1;
    isp1301_udc_configure(udc);
// Allocate memory for the UDCA
    udc.udca_v_base = dma_alloc_coherent(&pdev.dev, UDCA_BUFF_SIZE,
    &dma_handle,
    (GFP_KERNEL | GFP_DMA));
    if (!udc.udca_v_base) {
    dev_err(udc.dev, "error getting UDCA region\n");
    retval = -ENOMEM;
    goto err_disable_clk;
    }
    udc.udca_p_base = dma_handle;
    dev_dbg(udc.dev, "DMA buffer(0x%x bytes), P:0x%08x, V:0x%p\n",
    UDCA_BUFF_SIZE, udc.udca_p_base, udc.udca_v_base);
// Setup the DD DMA memory pool
    udc.dd_cache = dma_pool_create("udc_dd", udc.dev,
    sizeof(struct lpc32xx_usbd_dd_gad),
    sizeof(u32), 0);
    if (!udc.dd_cache) {
    dev_err(udc.dev, "error getting DD DMA region\n");
    retval = -ENOMEM;
    goto err_free_dma;
    }
// Clear USB peripheral and initialize gadget endpoints
    udc_disable(udc);
    udc_reinit(udc);
// Request IRQs - low and high priority USB device IRQs are routed to
// the same handler, while the DMA interrupt is routed elsewhere
    retval = devm_request_irq(dev, udc.udp_irq[IRQ_USB_LP],
    lpc32xx_usb_lp_irq, 0, "udc_lp", udc);
    if (retval < 0) {
    dev_err(udc.dev, "LP request irq %d failed\n",
    udc.udp_irq[IRQ_USB_LP]);
    goto err_destroy_pool;
    }
    retval = devm_request_irq(dev, udc.udp_irq[IRQ_USB_HP],
    lpc32xx_usb_hp_irq, 0, "udc_hp", udc);
    if (retval < 0) {
    dev_err(udc.dev, "HP request irq %d failed\n",
    udc.udp_irq[IRQ_USB_HP]);
    goto err_destroy_pool;
    }
    retval = devm_request_irq(dev, udc.udp_irq[IRQ_USB_DEVDMA],
    lpc32xx_usb_devdma_irq, 0, "udc_dma", udc);
    if (retval < 0) {
    dev_err(udc.dev, "DEV request irq %d failed\n",
    udc.udp_irq[IRQ_USB_DEVDMA]);
    goto err_destroy_pool;
    }
// The transceiver interrupt is used for VBUS detection and will
    kick off the VBUS handler function */
    retval = devm_request_threaded_irq(dev, udc.udp_irq[IRQ_USB_ATX], core::ptr::null_mut(),
    lpc32xx_usb_vbus_irq, IRQF_ONESHOT,
    "udc_otg", udc);
    if (retval < 0) {
    dev_err(udc.dev, "VBUS request irq %d failed\n",
    udc.udp_irq[IRQ_USB_ATX]);
    goto err_destroy_pool;
    }
// Initialize wait queue
    init_waitqueue_head(&udc.ep_disable_wait_queue);
    atomic_set(&udc.enabled_ep_cnt, 0);
    retval = usb_add_gadget_udc(dev, &udc.gadget);
    if (retval < 0)
    goto err_destroy_pool;
    dev_set_drvdata(dev, udc);
    device_init_wakeup(dev, 1);
    create_debug_file(udc);
// Disable clocks for now
    udc_clk_set(udc, 0);
    dev_info(udc.dev, "%s version %s\n", driver_name, DRIVER_VERSION);
    return 0;
    err_destroy_pool:
    dma_pool_destroy(udc.dd_cache);
    err_free_dma:
    dma_free_coherent(&pdev.dev, UDCA_BUFF_SIZE,
    udc.udca_v_base, udc.udca_p_base);
    err_disable_clk:
    clk_disable_unprepare(udc.usb_slv_clk);
    err_put_client:
    put_device(&udc.isp1301_i2c_client.dev);
    dev_err(udc.dev, "%s probe failed, %d\n", driver_name, retval);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_udc_remove(pdev: *mut platform_device) {
    static void lpc32xx_udc_remove(struct platform_device *pdev)
    {
    struct lpc32xx_udc *udc = platform_get_drvdata(pdev);
    usb_del_gadget_udc(&udc.gadget);
    if (udc.driver) {
    dev_err(&pdev.dev,
    "Driver still in use but removing anyhow\n");
    return;
    }
    udc_clk_set(udc, 1);
    udc_disable(udc);
    pullup(udc, 0);
    device_init_wakeup(&pdev.dev, 0);
    remove_debug_file(udc);
    dma_pool_destroy(udc.dd_cache);
    dma_free_coherent(&pdev.dev, UDCA_BUFF_SIZE,
    udc.udca_v_base, udc.udca_p_base);
    clk_disable_unprepare(udc.usb_slv_clk);
    put_device(&udc.isp1301_i2c_client.dev);
    }

#[no_mangle]
unsafe extern "C" fn lpc32xx_udc_suspend(pdev: *mut platform_device, mesg: pm_message_t) -> c_int {
    static int lpc32xx_udc_suspend(struct platform_device *pdev, pm_message_t mesg)
    {
    struct lpc32xx_udc *udc = platform_get_drvdata(pdev);
    if (udc.clocked) {
// Power down ISP
    udc.poweron = 0;
    isp1301_set_powerstate(udc, 0);
// Disable clocking
    udc_clk_set(udc, 0);
// Keep clock flag on, so we know to re-enable clocks
    on resume */
    udc.clocked = 1;
// Kill global USB clock
    clk_disable_unprepare(udc.usb_slv_clk);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_udc_resume(pdev: *mut platform_device) -> c_int {
    static int lpc32xx_udc_resume(struct platform_device *pdev)
    {
    struct lpc32xx_udc *udc = platform_get_drvdata(pdev);
    if (udc.clocked) {
// Enable global USB clock
    clk_prepare_enable(udc.usb_slv_clk);
// Enable clocking
    udc_clk_set(udc, 1);
// ISP back to normal power mode
    udc.poweron = 1;
    isp1301_set_powerstate(udc, 1);
    }
    return 0;
    }

    static const struct of_device_id lpc32xx_udc_of_match[] = {
    { .compatible = "nxp,lpc3220-udc", },
    { },
    };
    MODULE_DEVICE_TABLE(of, lpc32xx_udc_of_match);

    static struct platform_driver lpc32xx_udc_driver = {
    .probe		= lpc32xx_udc_probe,
    .remove		= lpc32xx_udc_remove,
    .shutdown	= lpc32xx_udc_shutdown,
    .suspend	= lpc32xx_udc_suspend,
    .resume		= lpc32xx_udc_resume,
    .driver		= {
    .name	= driver_name,
    .of_match_table = of_match_ptr(lpc32xx_udc_of_match),
    },
    };
    module_platform_driver(lpc32xx_udc_driver);
    MODULE_DESCRIPTION("LPC32XX udc driver");
    MODULE_AUTHOR("Kevin Wells <kevin.wells@nxp.com>");
    MODULE_AUTHOR("Roland Stigge <stigge@antcom.de>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:lpc32xx_udc");
