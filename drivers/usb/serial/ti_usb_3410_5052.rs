//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/ti_usb_3410_5052.c
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
// TI 3410/5052 USB Serial Driver
//
// Copyright (C) 2004 Texas Instruments
//
// This driver is based on the Linux io_ti driver, which is
// Copyright (C) 2000-2002 Inside Out Networks
// Copyright (C) 2001-2002 Greg Kroah-Hartman
//
// For questions or problems with this driver, contact Texas Instruments
// technical support, or Al Borchers <alborchers@steinerpoint.com>, or
// Peter Berger <pberger@brimson.com>.
//

// Configuration ids
pub const TI_BOOT_CONFIG: c_int = 1;
pub const TI_ACTIVE_CONFIG: c_int = 2;
// Vendor and product ids
pub const TI_VENDOR_ID: c_uint = 0x0451;
pub const IBM_VENDOR_ID: c_uint = 0x04b3;
pub const STARTECH_VENDOR_ID: c_uint = 0x14b0;
pub const TI_3410_PRODUCT_ID: c_uint = 0x3410;
pub const IBM_4543_PRODUCT_ID: c_uint = 0x4543;
pub const IBM_454B_PRODUCT_ID: c_uint = 0x454b;
pub const IBM_454C_PRODUCT_ID: c_uint = 0x454c;
pub const TI_3410_EZ430_ID: c_uint = 0xF430  /* TI ez430 development tool */;
pub const TI_5052_BOOT_PRODUCT_ID: c_uint = 0x5052	/* no EEPROM, no firmware */;
pub const TI_5152_BOOT_PRODUCT_ID: c_uint = 0x5152	/* no EEPROM, no firmware */;
pub const TI_5052_EEPROM_PRODUCT_ID: c_uint = 0x505A	/* EEPROM, no firmware */;
pub const TI_5052_FIRMWARE_PRODUCT_ID: c_uint = 0x505F	/* firmware is running */;
pub const FRI2_PRODUCT_ID: c_uint = 0x5053  /* Fish River Island II */;
// Multi-Tech vendor and product ids
pub const MTS_VENDOR_ID: c_uint = 0x06E0;
pub const MTS_GSM_NO_FW_PRODUCT_ID: c_uint = 0xF108;
pub const MTS_CDMA_NO_FW_PRODUCT_ID: c_uint = 0xF109;
pub const MTS_CDMA_PRODUCT_ID: c_uint = 0xF110;
pub const MTS_GSM_PRODUCT_ID: c_uint = 0xF111;
pub const MTS_EDGE_PRODUCT_ID: c_uint = 0xF112;
pub const MTS_MT9234MU_PRODUCT_ID: c_uint = 0xF114;
pub const MTS_MT9234ZBA_PRODUCT_ID: c_uint = 0xF115;
pub const MTS_MT9234ZBAOLD_PRODUCT_ID: c_uint = 0x0319;
// Abbott Diabetics vendor and product ids
pub const ABBOTT_VENDOR_ID: c_uint = 0x1a61;
pub const ABBOTT_STEREO_PLUG_ID: c_uint = 0x3410;

pub const ABBOTT_STRIP_PORT_ID: c_uint = 0x3420;
// Honeywell vendor and product IDs
pub const HONEYWELL_VENDOR_ID: c_uint = 0x10ac;
pub const HONEYWELL_HGI80_PRODUCT_ID: c_uint = 0x0102  /* Honeywell HGI80 */;
// Moxa UPORT 11x0 vendor and product IDs
pub const MXU1_VENDOR_ID: c_uint = 0x110a;
pub const MXU1_1110_PRODUCT_ID: c_uint = 0x1110;
pub const MXU1_1130_PRODUCT_ID: c_uint = 0x1130;
pub const MXU1_1150_PRODUCT_ID: c_uint = 0x1150;
pub const MXU1_1151_PRODUCT_ID: c_uint = 0x1151;
pub const MXU1_1131_PRODUCT_ID: c_uint = 0x1131;
// Commands
pub const TI_GET_VERSION: c_uint = 0x01;
pub const TI_GET_PORT_STATUS: c_uint = 0x02;
pub const TI_GET_PORT_DEV_INFO: c_uint = 0x03;
pub const TI_GET_CONFIG: c_uint = 0x04;
pub const TI_SET_CONFIG: c_uint = 0x05;
pub const TI_OPEN_PORT: c_uint = 0x06;
pub const TI_CLOSE_PORT: c_uint = 0x07;
pub const TI_START_PORT: c_uint = 0x08;
pub const TI_STOP_PORT: c_uint = 0x09;
pub const TI_TEST_PORT: c_uint = 0x0A;
pub const TI_PURGE_PORT: c_uint = 0x0B;
pub const TI_RESET_EXT_DEVICE: c_uint = 0x0C;
pub const TI_WRITE_DATA: c_uint = 0x80;
pub const TI_READ_DATA: c_uint = 0x81;
pub const TI_REQ_TYPE_CLASS: c_uint = 0x82;
// Module identifiers
pub const TI_I2C_PORT: c_uint = 0x01;
pub const TI_IEEE1284_PORT: c_uint = 0x02;
pub const TI_UART1_PORT: c_uint = 0x03;
pub const TI_UART2_PORT: c_uint = 0x04;
pub const TI_RAM_PORT: c_uint = 0x05;
// Modem status
pub const TI_MSR_DELTA_CTS: c_uint = 0x01;
pub const TI_MSR_DELTA_DSR: c_uint = 0x02;
pub const TI_MSR_DELTA_RI: c_uint = 0x04;
pub const TI_MSR_DELTA_CD: c_uint = 0x08;
pub const TI_MSR_CTS: c_uint = 0x10;
pub const TI_MSR_DSR: c_uint = 0x20;
pub const TI_MSR_RI: c_uint = 0x40;
pub const TI_MSR_CD: c_uint = 0x80;
pub const TI_MSR_DELTA_MASK: c_uint = 0x0F;
pub const TI_MSR_MASK: c_uint = 0xF0;
// Line status
pub const TI_LSR_OVERRUN_ERROR: c_uint = 0x01;
pub const TI_LSR_PARITY_ERROR: c_uint = 0x02;
pub const TI_LSR_FRAMING_ERROR: c_uint = 0x04;
pub const TI_LSR_BREAK: c_uint = 0x08;
pub const TI_LSR_ERROR: c_uint = 0x0F;
pub const TI_LSR_RX_FULL: c_uint = 0x10;
pub const TI_LSR_TX_EMPTY: c_uint = 0x20;
pub const TI_LSR_TX_EMPTY_BOTH: c_uint = 0x40;
// Line control
pub const TI_LCR_BREAK: c_uint = 0x40;
// Modem control
pub const TI_MCR_LOOP: c_uint = 0x04;
pub const TI_MCR_DTR: c_uint = 0x10;
pub const TI_MCR_RTS: c_uint = 0x20;
// Mask settings
pub const TI_UART_ENABLE_RTS_IN: c_uint = 0x0001;
pub const TI_UART_DISABLE_RTS: c_uint = 0x0002;
pub const TI_UART_ENABLE_PARITY_CHECKING: c_uint = 0x0008;
pub const TI_UART_ENABLE_DSR_OUT: c_uint = 0x0010;
pub const TI_UART_ENABLE_CTS_OUT: c_uint = 0x0020;
pub const TI_UART_ENABLE_X_OUT: c_uint = 0x0040;
pub const TI_UART_ENABLE_XA_OUT: c_uint = 0x0080;
pub const TI_UART_ENABLE_X_IN: c_uint = 0x0100;
pub const TI_UART_ENABLE_DTR_IN: c_uint = 0x0800;
pub const TI_UART_DISABLE_DTR: c_uint = 0x1000;
pub const TI_UART_ENABLE_MS_INTS: c_uint = 0x2000;
pub const TI_UART_ENABLE_AUTO_START_DMA: c_uint = 0x4000;
// Parity
pub const TI_UART_NO_PARITY: c_uint = 0x00;
pub const TI_UART_ODD_PARITY: c_uint = 0x01;
pub const TI_UART_EVEN_PARITY: c_uint = 0x02;
pub const TI_UART_MARK_PARITY: c_uint = 0x03;
pub const TI_UART_SPACE_PARITY: c_uint = 0x04;
// Stop bits
pub const TI_UART_1_STOP_BITS: c_uint = 0x00;
pub const TI_UART_1_5_STOP_BITS: c_uint = 0x01;
pub const TI_UART_2_STOP_BITS: c_uint = 0x02;
// Bits per character
pub const TI_UART_5_DATA_BITS: c_uint = 0x00;
pub const TI_UART_6_DATA_BITS: c_uint = 0x01;
pub const TI_UART_7_DATA_BITS: c_uint = 0x02;
pub const TI_UART_8_DATA_BITS: c_uint = 0x03;
// 232/485 modes
pub const TI_UART_232: c_uint = 0x00;
pub const TI_UART_485_RECEIVER_DISABLED: c_uint = 0x01;
pub const TI_UART_485_RECEIVER_ENABLED: c_uint = 0x02;
// Pipe transfer mode and timeout
pub const TI_PIPE_MODE_CONTINUOUS: c_uint = 0x01;
pub const TI_PIPE_MODE_MASK: c_uint = 0x03;
pub const TI_PIPE_TIMEOUT_MASK: c_uint = 0x7C;
pub const TI_PIPE_TIMEOUT_ENABLE: c_uint = 0x80;
// Config struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_uart_config {
    pub wBaudRate: __be16,
    pub wFlags: __be16,
    pub bDataBits: u8,
    pub bParity: u8,
    pub bStopBits: u8,
    pub cXon: c_char,
    pub cXoff: c_char,
    pub bUartMode: u8,
}

// Get port status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_port_status {
    pub bCmdCode: u8,
    pub bModuleId: u8,
    pub bErrorCode: u8,
    pub bMSR: u8,
    pub bLSR: u8,
}

// Purge modes
pub const TI_PURGE_OUTPUT: c_uint = 0x00;
pub const TI_PURGE_INPUT: c_uint = 0x80;
// Read/Write data
pub const TI_RW_DATA_ADDR_SFR: c_uint = 0x10;
pub const TI_RW_DATA_ADDR_IDATA: c_uint = 0x20;
pub const TI_RW_DATA_ADDR_XDATA: c_uint = 0x30;
pub const TI_RW_DATA_ADDR_CODE: c_uint = 0x40;
pub const TI_RW_DATA_ADDR_GPIO: c_uint = 0x50;
pub const TI_RW_DATA_ADDR_I2C: c_uint = 0x60;
pub const TI_RW_DATA_ADDR_FLASH: c_uint = 0x70;
pub const TI_RW_DATA_ADDR_DSP: c_uint = 0x80;
pub const TI_RW_DATA_UNSPECIFIED: c_uint = 0x00;
pub const TI_RW_DATA_BYTE: c_uint = 0x01;
pub const TI_RW_DATA_WORD: c_uint = 0x02;
pub const TI_RW_DATA_DOUBLE_WORD: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_write_data_bytes {
    pub bAddrType: u8,
    pub bDataType: u8,
    pub bDataCounter: u8,
    pub wBaseAddrHi: __be16,
    pub wBaseAddrLo: __be16,
    pub bData: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_read_data_request {
    pub bAddrType: u8,
    pub bDataType: u8,
    pub bDataCounter: u8,
    pub wBaseAddrHi: __be16,
    pub wBaseAddrLo: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_read_data_bytes {
    pub bCmdCode: u8,
    pub bModuleId: u8,
    pub bErrorCode: u8,
    pub bData: [u8; ],
}

// Interrupt struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_interrupt {
    pub bICode: u8,
    pub bIInfo: u8,
}

// Interrupt codes
pub const TI_CODE_HARDWARE_ERROR: c_uint = 0xFF;
pub const TI_CODE_DATA_ERROR: c_uint = 0x03;
pub const TI_CODE_MODEM_STATUS: c_uint = 0x04;
// Download firmware max packet size
pub const TI_DOWNLOAD_MAX_PACKET_SIZE: c_int = 64;
// Firmware image header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_firmware_header {
    pub wLength: __le16,
    pub bCheckSum: u8,
    pub __packed: },
// UART addresses
pub const TI_UART1_BASE_ADDR: c_uint = 0xFFA0	/* UART 1 base address */;
pub const TI_UART2_BASE_ADDR: c_uint = 0xFFB0	/* UART 2 base address */;
pub const TI_UART_OFFSET_LCR: c_uint = 0x0002	/* UART MCR register offset */;
pub const TI_UART_OFFSET_MCR: c_uint = 0x0004	/* UART MCR register offset */;

pub const TI_FIRMWARE_BUF_SIZE: c_int = 16284;
pub const TI_TRANSFER_TIMEOUT: c_int = 2;
// read urb states
pub const TI_READ_URB_RUNNING: c_int = 0;
pub const TI_READ_URB_STOPPING: c_int = 1;
pub const TI_READ_URB_STOPPED: c_int = 2;
pub const TI_EXTRA_VID_PID_COUNT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_port {
    pub tp_is_open: c_int,
    pub tp_msr: u8,
    pub tp_shadow_mcr: u8,
    pub /: *mut *mut u8 tp_uart_mode; / 232 or 485 modes,
    pub tp_uart_base_addr: c_uint,
    pub tp_tdev: *mut ti_device,
    pub tp_port: *mut usb_serial_port,
    pub tp_lock: spinlock_t,
    pub tp_read_urb_state: c_int,
    pub tp_write_urb_in_use: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_device {
    pub td_open_close_lock: mutex,
    pub td_open_port_count: c_int,
    pub td_serial: *mut usb_serial,
    pub td_is_3410: c_int,
    pub td_rs485_only: bool,
}

    static int ti_startup(struct usb_serial *serial);
    static void ti_release(struct usb_serial *serial);
    static int ti_port_probe(struct usb_serial_port *port);
    static void ti_port_remove(struct usb_serial_port *port);
    static int ti_open(struct tty_struct *tty, struct usb_serial_port *port);
    static void ti_close(struct usb_serial_port *port);
    static int ti_write(struct tty_struct *tty, struct usb_serial_port *port,
    const unsigned char *data, int count);
    static unsigned int ti_write_room(struct tty_struct *tty);
    static unsigned int ti_chars_in_buffer(struct tty_struct *tty);
    static bool ti_tx_empty(struct usb_serial_port *port);
    static void ti_throttle(struct tty_struct *tty);
    static void ti_unthrottle(struct tty_struct *tty);
    static void ti_set_termios(struct tty_struct *tty,
    struct usb_serial_port *port,
    const struct ktermios *old_termios);
    static int ti_tiocmget(struct tty_struct *tty);
    static int ti_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear);
    static int ti_break(struct tty_struct *tty, int break_state);
    static void ti_interrupt_callback(struct urb *urb);
    static void ti_bulk_in_callback(struct urb *urb);
    static void ti_bulk_out_callback(struct urb *urb);
    static void ti_recv(struct usb_serial_port *port, unsigned char *data,
    int length);
    static void ti_send(struct ti_port *tport);
    static int ti_set_mcr(struct ti_port *tport, unsigned int mcr);
    static int ti_get_lsr(struct ti_port *tport, u8 *lsr);
    static void ti_get_serial_info(struct tty_struct *tty, struct serial_struct *ss);
    static void ti_handle_new_msr(struct ti_port *tport, u8 msr);
    static void ti_stop_read(struct ti_port *tport, struct tty_struct *tty);
    static int ti_restart_read(struct ti_port *tport, struct tty_struct *tty);
    static int ti_command_out_sync(struct usb_device *udev, u8 command,
    u16 moduleid, u16 value, void *data, int size);
    static int ti_command_in_sync(struct usb_device *udev, u8 command,
    u16 moduleid, u16 value, void *data, int size);
    static int ti_port_cmd_out(struct usb_serial_port *port, u8 command,
    u16 value, void *data, int size);
    static int ti_port_cmd_in(struct usb_serial_port *port, u8 command,
    u16 value, void *data, int size);
    static int ti_write_byte(struct usb_serial_port *port, struct ti_device *tdev,
    unsigned long addr, u8 mask, u8 byte);
    static int ti_download_firmware(struct ti_device *tdev);
    static const struct usb_device_id ti_id_table_3410[] = {
    { USB_DEVICE(TI_VENDOR_ID, TI_3410_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_3410_EZ430_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_GSM_NO_FW_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_CDMA_NO_FW_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_CDMA_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_GSM_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_EDGE_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_MT9234MU_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_MT9234ZBA_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_MT9234ZBAOLD_PRODUCT_ID) },
    { USB_DEVICE(IBM_VENDOR_ID, IBM_4543_PRODUCT_ID) },
    { USB_DEVICE(IBM_VENDOR_ID, IBM_454B_PRODUCT_ID) },
    { USB_DEVICE(IBM_VENDOR_ID, IBM_454C_PRODUCT_ID) },
    { USB_DEVICE(ABBOTT_VENDOR_ID, ABBOTT_STEREO_PLUG_ID) },
    { USB_DEVICE(ABBOTT_VENDOR_ID, ABBOTT_STRIP_PORT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, FRI2_PRODUCT_ID) },
    { USB_DEVICE(HONEYWELL_VENDOR_ID, HONEYWELL_HGI80_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1110_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1130_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1131_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1150_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1151_PRODUCT_ID) },
    { USB_DEVICE(STARTECH_VENDOR_ID, TI_3410_PRODUCT_ID) },
    { }	/* terminator */
    };
    static const struct usb_device_id ti_id_table_5052[] = {
    { USB_DEVICE(TI_VENDOR_ID, TI_5052_BOOT_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_5152_BOOT_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_5052_EEPROM_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_5052_FIRMWARE_PRODUCT_ID) },
    { }
    };
    static const struct usb_device_id ti_id_table_combined[] = {
    { USB_DEVICE(TI_VENDOR_ID, TI_3410_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_3410_EZ430_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_GSM_NO_FW_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_CDMA_NO_FW_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_CDMA_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_GSM_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_EDGE_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_MT9234MU_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_MT9234ZBA_PRODUCT_ID) },
    { USB_DEVICE(MTS_VENDOR_ID, MTS_MT9234ZBAOLD_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_5052_BOOT_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_5152_BOOT_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_5052_EEPROM_PRODUCT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, TI_5052_FIRMWARE_PRODUCT_ID) },
    { USB_DEVICE(IBM_VENDOR_ID, IBM_4543_PRODUCT_ID) },
    { USB_DEVICE(IBM_VENDOR_ID, IBM_454B_PRODUCT_ID) },
    { USB_DEVICE(IBM_VENDOR_ID, IBM_454C_PRODUCT_ID) },
    { USB_DEVICE(ABBOTT_VENDOR_ID, ABBOTT_PRODUCT_ID) },
    { USB_DEVICE(ABBOTT_VENDOR_ID, ABBOTT_STRIP_PORT_ID) },
    { USB_DEVICE(TI_VENDOR_ID, FRI2_PRODUCT_ID) },
    { USB_DEVICE(HONEYWELL_VENDOR_ID, HONEYWELL_HGI80_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1110_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1130_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1131_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1150_PRODUCT_ID) },
    { USB_DEVICE(MXU1_VENDOR_ID, MXU1_1151_PRODUCT_ID) },
    { USB_DEVICE(STARTECH_VENDOR_ID, TI_3410_PRODUCT_ID) },
    { }	/* terminator */
    };
    static struct usb_serial_driver ti_1port_device = {
    .driver = {
    .name		= "ti_usb_3410_5052_1",
    },
    .description		= "TI USB 3410 1 port adapter",
    .id_table		= ti_id_table_3410,
    .num_ports		= 1,
    .num_bulk_out		= 1,
    .attach			= ti_startup,
    .release		= ti_release,
    .port_probe		= ti_port_probe,
    .port_remove		= ti_port_remove,
    .open			= ti_open,
    .close			= ti_close,
    .write			= ti_write,
    .write_room		= ti_write_room,
    .chars_in_buffer	= ti_chars_in_buffer,
    .tx_empty		= ti_tx_empty,
    .throttle		= ti_throttle,
    .unthrottle		= ti_unthrottle,
    .get_serial		= ti_get_serial_info,
    .set_termios		= ti_set_termios,
    .tiocmget		= ti_tiocmget,
    .tiocmset		= ti_tiocmset,
    .tiocmiwait		= usb_serial_generic_tiocmiwait,
    .get_icount		= usb_serial_generic_get_icount,
    .break_ctl		= ti_break,
    .read_int_callback	= ti_interrupt_callback,
    .read_bulk_callback	= ti_bulk_in_callback,
    .write_bulk_callback	= ti_bulk_out_callback,
    };
    static struct usb_serial_driver ti_2port_device = {
    .driver = {
    .name		= "ti_usb_3410_5052_2",
    },
    .description		= "TI USB 5052 2 port adapter",
    .id_table		= ti_id_table_5052,
    .num_ports		= 2,
    .num_bulk_out		= 1,
    .attach			= ti_startup,
    .release		= ti_release,
    .port_probe		= ti_port_probe,
    .port_remove		= ti_port_remove,
    .open			= ti_open,
    .close			= ti_close,
    .write			= ti_write,
    .write_room		= ti_write_room,
    .chars_in_buffer	= ti_chars_in_buffer,
    .tx_empty		= ti_tx_empty,
    .throttle		= ti_throttle,
    .unthrottle		= ti_unthrottle,
    .get_serial		= ti_get_serial_info,
    .set_termios		= ti_set_termios,
    .tiocmget		= ti_tiocmget,
    .tiocmset		= ti_tiocmset,
    .tiocmiwait		= usb_serial_generic_tiocmiwait,
    .get_icount		= usb_serial_generic_get_icount,
    .break_ctl		= ti_break,
    .read_int_callback	= ti_interrupt_callback,
    .read_bulk_callback	= ti_bulk_in_callback,
    .write_bulk_callback	= ti_bulk_out_callback,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &ti_1port_device, &ti_2port_device, core::ptr::null_mut()
    };
    MODULE_AUTHOR(TI_DRIVER_AUTHOR);
    MODULE_DESCRIPTION(TI_DRIVER_DESC);
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("ti_3410.fw");
    MODULE_FIRMWARE("ti_5052.fw");
    MODULE_FIRMWARE("mts_cdma.fw");
    MODULE_FIRMWARE("mts_gsm.fw");
    MODULE_FIRMWARE("mts_edge.fw");
    MODULE_FIRMWARE("mts_mt9234mu.fw");
    MODULE_FIRMWARE("mts_mt9234zba.fw");
    MODULE_FIRMWARE("moxa/moxa-1110.fw");
    MODULE_FIRMWARE("moxa/moxa-1130.fw");
    MODULE_FIRMWARE("moxa/moxa-1131.fw");
    MODULE_FIRMWARE("moxa/moxa-1150.fw");
    MODULE_FIRMWARE("moxa/moxa-1151.fw");
    MODULE_DEVICE_TABLE(usb, ti_id_table_combined);
    module_usb_serial_driver(serial_drivers, ti_id_table_combined);
#[no_mangle]
unsafe extern "C" fn ti_startup(serial: *mut usb_serial) -> c_int {
    static int ti_startup(struct usb_serial *serial)
    {
    struct ti_device *tdev;
    struct usb_device *dev = serial.dev;
    struct usb_host_interface *cur_altsetting;
    int num_endpoints;
    u16 vid, pid;
    int status;
    dev_dbg(&dev.dev,
    "%s - product 0x%4X, num configurations %d, configuration value %d\n",
    __func__, le16_to_cpu(dev.descriptor.idProduct),
    dev.descriptor.bNumConfigurations,
    dev.actconfig.desc.bConfigurationValue);
    tdev = kzalloc_obj(struct ti_device);
    if (!tdev)
    return -ENOMEM;
    mutex_init(&tdev.td_open_close_lock);
    tdev.td_serial = serial;
    usb_set_serial_data(serial, tdev);
// determine device type
    if (serial.type == &ti_1port_device)
    tdev.td_is_3410 = 1;
    dev_dbg(&dev.dev, "%s - device type is %s\n", __func__,
    tdev.td_is_3410 ? "3410" : "5052");
    vid = le16_to_cpu(dev.descriptor.idVendor);
    pid = le16_to_cpu(dev.descriptor.idProduct);
    if (vid == MXU1_VENDOR_ID) {
    switch (pid) {
    case MXU1_1130_PRODUCT_ID:
    case MXU1_1131_PRODUCT_ID:
    tdev.td_rs485_only = true;
    break;
    }
    }
    cur_altsetting = serial.interface.cur_altsetting;
    num_endpoints = cur_altsetting.desc.bNumEndpoints;
// if we have only 1 configuration and 1 endpoint, download firmware
    if (dev.descriptor.bNumConfigurations == 1 && num_endpoints == 1) {
    status = ti_download_firmware(tdev);
    if (status != 0)
    goto free_tdev;
// 3410 must be reset, 5052 resets itself
    if (tdev.td_is_3410) {
    msleep_interruptible(100);
    usb_reset_device(dev);
    }
    status = -ENODEV;
    goto free_tdev;
    }
// the second configuration must be set
    if (dev.actconfig.desc.bConfigurationValue == TI_BOOT_CONFIG) {
    status = usb_driver_set_configuration(dev, TI_ACTIVE_CONFIG);
    status = status ? status : -ENODEV;
    goto free_tdev;
    }
    if (serial.num_bulk_in < serial.num_ports ||
    serial.num_bulk_out < serial.num_ports) {
    dev_err(&serial.interface.dev, "missing endpoints\n");
    status = -ENODEV;
    goto free_tdev;
    }
    return 0;
    free_tdev:
    kfree(tdev);
    usb_set_serial_data(serial, core::ptr::null_mut());
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ti_release(serial: *mut usb_serial) {
    static void ti_release(struct usb_serial *serial)
    {
    struct ti_device *tdev = usb_get_serial_data(serial);
    kfree(tdev);
    }
#[no_mangle]
unsafe extern "C" fn ti_port_probe(port: *mut usb_serial_port) -> c_int {
    static int ti_port_probe(struct usb_serial_port *port)
    {
    struct ti_port *tport;
    tport = kzalloc_obj(*tport);
    if (!tport)
    return -ENOMEM;
    spin_lock_init(&tport.tp_lock);
    if (port == port.serial.port[0])
    tport.tp_uart_base_addr = TI_UART1_BASE_ADDR;
    else
    tport.tp_uart_base_addr = TI_UART2_BASE_ADDR;
    tport.tp_port = port;
    tport.tp_tdev = usb_get_serial_data(port.serial);
    if (tport.tp_tdev.td_rs485_only)
    tport.tp_uart_mode = TI_UART_485_RECEIVER_DISABLED;
    else
    tport.tp_uart_mode = TI_UART_232;
    usb_set_serial_port_data(port, tport);
//
// The TUSB5052 LSR does not tell when the transmitter shift register
// has emptied so add a one-character drain delay.
//
    if (!tport.tp_tdev.td_is_3410)
    port.port.drain_delay = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_port_remove(port: *mut usb_serial_port) {
    static void ti_port_remove(struct usb_serial_port *port)
    {
    struct ti_port *tport;
    tport = usb_get_serial_port_data(port);
    kfree(tport);
    }
#[no_mangle]
unsafe extern "C" fn ti_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int ti_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    struct ti_port *tport = usb_get_serial_port_data(port);
    struct ti_device *tdev;
    struct usb_device *dev;
    struct urb *urb;
    int status;
    u16 open_settings;
    open_settings = (TI_PIPE_MODE_CONTINUOUS |
    TI_PIPE_TIMEOUT_ENABLE |
    (TI_TRANSFER_TIMEOUT << 2));
    dev = port.serial.dev;
    tdev = tport.tp_tdev;
// only one open on any port on a device at a time
    if (mutex_lock_interruptible(&tdev.td_open_close_lock))
    return -ERESTARTSYS;
    tport.tp_msr = 0;
    tport.tp_shadow_mcr |= (TI_MCR_RTS | TI_MCR_DTR);
// start interrupt urb the first time a port is opened on this device
    if (tdev.td_open_port_count == 0) {
    dev_dbg(&port.dev, "%s - start interrupt in urb\n", __func__);
    urb = tdev.td_serial.port[0].interrupt_in_urb;
    if (!urb) {
    dev_err(&port.dev, "%s - no interrupt urb\n", __func__);
    status = -EINVAL;
    goto release_lock;
    }
    urb.context = tdev;
    status = usb_submit_urb(urb, GFP_KERNEL);
    if (status) {
    dev_err(&port.dev, "%s - submit interrupt urb failed, %d\n", __func__, status);
    goto release_lock;
    }
    }
    if (tty)
    ti_set_termios(tty, port, &tty.termios);
    status = ti_port_cmd_out(port, TI_OPEN_PORT, open_settings, core::ptr::null_mut(), 0);
    if (status) {
    dev_err(&port.dev, "%s - cannot send open command, %d\n",
    __func__, status);
    goto unlink_int_urb;
    }
    status = ti_port_cmd_out(port, TI_START_PORT, 0, core::ptr::null_mut(), 0);
    if (status) {
    dev_err(&port.dev, "%s - cannot send start command, %d\n",
    __func__, status);
    goto unlink_int_urb;
    }
    status = ti_port_cmd_out(port, TI_PURGE_PORT, TI_PURGE_INPUT, core::ptr::null_mut(), 0);
    if (status) {
    dev_err(&port.dev, "%s - cannot clear input buffers, %d\n",
    __func__, status);
    goto unlink_int_urb;
    }
    status = ti_port_cmd_out(port, TI_PURGE_PORT, TI_PURGE_OUTPUT, core::ptr::null_mut(), 0);
    if (status) {
    dev_err(&port.dev, "%s - cannot clear output buffers, %d\n",
    __func__, status);
    goto unlink_int_urb;
    }
// reset the data toggle on the bulk endpoints to work around bug in
// host controllers where things get out of sync some times
    usb_clear_halt(dev, port.write_urb.pipe);
    usb_clear_halt(dev, port.read_urb.pipe);
    if (tty)
    ti_set_termios(tty, port, &tty.termios);
    status = ti_port_cmd_out(port, TI_OPEN_PORT, open_settings, core::ptr::null_mut(), 0);
    if (status) {
    dev_err(&port.dev, "%s - cannot send open command (2), %d\n",
    __func__, status);
    goto unlink_int_urb;
    }
    status = ti_port_cmd_out(port, TI_START_PORT, 0, core::ptr::null_mut(), 0);
    if (status) {
    dev_err(&port.dev, "%s - cannot send start command (2), %d\n",
    __func__, status);
    goto unlink_int_urb;
    }
// start read urb
    urb = port.read_urb;
    tport.tp_read_urb_state = TI_READ_URB_RUNNING;
    urb.context = tport;
    status = usb_submit_urb(urb, GFP_KERNEL);
    if (status) {
    dev_err(&port.dev, "%s - submit read urb failed, %d\n",
    __func__, status);
    goto unlink_int_urb;
    }
    tport.tp_is_open = 1;
    ++tdev.td_open_port_count;
    goto release_lock;
    unlink_int_urb:
    if (tdev.td_open_port_count == 0)
    usb_kill_urb(port.serial.port[0].interrupt_in_urb);
    release_lock:
    mutex_unlock(&tdev.td_open_close_lock);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ti_close(port: *mut usb_serial_port) {
    static void ti_close(struct usb_serial_port *port)
    {
    struct ti_device *tdev;
    struct ti_port *tport;
    int status;
    unsigned long flags;
    tdev = usb_get_serial_data(port.serial);
    tport = usb_get_serial_port_data(port);
    tport.tp_is_open = 0;
    usb_kill_urb(port.read_urb);
    usb_kill_urb(port.write_urb);
    tport.tp_write_urb_in_use = 0;
    spin_lock_irqsave(&tport.tp_lock, flags);
    kfifo_reset_out(&port.write_fifo);
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    status = ti_port_cmd_out(port, TI_CLOSE_PORT, 0, core::ptr::null_mut(), 0);
    if (status)
    dev_err(&port.dev,
    "%s - cannot send close port command, %d\n"
    , __func__, status);
    mutex_lock(&tdev.td_open_close_lock);
    --tdev.td_open_port_count;
    if (tdev.td_open_port_count == 0) {
// last port is closed, shut down interrupt urb
    usb_kill_urb(port.serial.port[0].interrupt_in_urb);
    }
    mutex_unlock(&tdev.td_open_close_lock);
    }
    static int ti_write(struct tty_struct *tty, struct usb_serial_port *port,
    const unsigned char *data, int count)
    {
    struct ti_port *tport = usb_get_serial_port_data(port);
    if (count == 0) {
    return 0;
    }
    if (!tport.tp_is_open)
    return -ENODEV;
    count = kfifo_in_locked(&port.write_fifo, data, count,
    &tport.tp_lock);
    ti_send(tport);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn ti_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int ti_write_room(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct ti_port *tport = usb_get_serial_port_data(port);
    unsigned int room;
    unsigned long flags;
    spin_lock_irqsave(&tport.tp_lock, flags);
    room = kfifo_avail(&port.write_fifo);
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    dev_dbg(&port.dev, "%s - returns %u\n", __func__, room);
    return room;
    }
#[no_mangle]
unsafe extern "C" fn ti_chars_in_buffer(tty: *mut tty_struct) -> c_uint {
    static unsigned int ti_chars_in_buffer(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct ti_port *tport = usb_get_serial_port_data(port);
    unsigned int chars;
    unsigned long flags;
    spin_lock_irqsave(&tport.tp_lock, flags);
    chars = kfifo_len(&port.write_fifo);
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    dev_dbg(&port.dev, "%s - returns %u\n", __func__, chars);
    return chars;
    }
#[no_mangle]
unsafe extern "C" fn ti_tx_empty(port: *mut usb_serial_port) -> bool {
    static bool ti_tx_empty(struct usb_serial_port *port)
    {
    struct ti_port *tport = usb_get_serial_port_data(port);
    u8 lsr, mask;
    int ret;
//
// TUSB5052 does not have the TEMT bit to tell if the shift register
// is empty.
//
    if (tport.tp_tdev.td_is_3410)
    mask = TI_LSR_TX_EMPTY_BOTH;
    else
    mask = TI_LSR_TX_EMPTY;
    ret = ti_get_lsr(tport, &lsr);
    if (!ret && !(lsr & mask))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ti_throttle(tty: *mut tty_struct) {
    static void ti_throttle(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct ti_port *tport = usb_get_serial_port_data(port);
    if (I_IXOFF(tty) || C_CRTSCTS(tty))
    ti_stop_read(tport, tty);
    }
#[no_mangle]
unsafe extern "C" fn ti_unthrottle(tty: *mut tty_struct) {
    static void ti_unthrottle(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct ti_port *tport = usb_get_serial_port_data(port);
    int status;
    if (I_IXOFF(tty) || C_CRTSCTS(tty)) {
    status = ti_restart_read(tport, tty);
    if (status)
    dev_err(&port.dev, "%s - cannot restart read, %d\n",
    __func__, status);
    }
    }
    static void ti_set_termios(struct tty_struct *tty,
    struct usb_serial_port *port,
    const struct ktermios *old_termios)
    {
    struct ti_port *tport = usb_get_serial_port_data(port);
    struct ti_uart_config *config;
    int baud;
    int status;
    unsigned int mcr;
    u16 wbaudrate;
    let mut wflags: u16 = 0;
    config = kmalloc_obj(*config);
    if (!config)
    return;
// these flags must be set
    wflags |= TI_UART_ENABLE_MS_INTS;
    wflags |= TI_UART_ENABLE_AUTO_START_DMA;
    config.bUartMode = tport.tp_uart_mode;
    switch (C_CSIZE(tty)) {
    case CS5:
    config.bDataBits = TI_UART_5_DATA_BITS;
    break;
    case CS6:
    config.bDataBits = TI_UART_6_DATA_BITS;
    break;
    case CS7:
    config.bDataBits = TI_UART_7_DATA_BITS;
    break;
    default:
    case CS8:
    config.bDataBits = TI_UART_8_DATA_BITS;
    break;
    }
// CMSPAR isn't supported by this driver
    tty.termios.c_cflag &= ~CMSPAR;
    if (C_PARENB(tty)) {
    if (C_PARODD(tty)) {
    wflags |= TI_UART_ENABLE_PARITY_CHECKING;
    config.bParity = TI_UART_ODD_PARITY;
    } else {
    wflags |= TI_UART_ENABLE_PARITY_CHECKING;
    config.bParity = TI_UART_EVEN_PARITY;
    }
    } else {
    wflags &= ~TI_UART_ENABLE_PARITY_CHECKING;
    config.bParity = TI_UART_NO_PARITY;
    }
    if (C_CSTOPB(tty))
    config.bStopBits = TI_UART_2_STOP_BITS;
    else
    config.bStopBits = TI_UART_1_STOP_BITS;
    if (C_CRTSCTS(tty)) {
// RTS flow control must be off to drop RTS for baud rate B0
    if ((C_BAUD(tty)) != B0)
    wflags |= TI_UART_ENABLE_RTS_IN;
    wflags |= TI_UART_ENABLE_CTS_OUT;
    } else {
    ti_restart_read(tport, tty);
    }
    if (I_IXOFF(tty) || I_IXON(tty)) {
    config.cXon  = START_CHAR(tty);
    config.cXoff = STOP_CHAR(tty);
    if (I_IXOFF(tty))
    wflags |= TI_UART_ENABLE_X_IN;
    else
    ti_restart_read(tport, tty);
    if (I_IXON(tty))
    wflags |= TI_UART_ENABLE_X_OUT;
    }
    baud = tty_get_baud_rate(tty);
    if (!baud)
    baud = 9600;
    if (tport.tp_tdev.td_is_3410)
    wbaudrate = (923077 + baud/2) / baud;
    else
    wbaudrate = (461538 + baud/2) / baud;
// FIXME: Should calculate resulting baud here and report it back
    if ((C_BAUD(tty)) != B0)
    tty_encode_baud_rate(tty, baud, baud);
    dev_dbg(&port.dev,
    "%s - BaudRate=%d, wBaudRate=%d, wFlags=0x%04X, bDataBits=%d, bParity=%d, bStopBits=%d, cXon=%d, cXoff=%d, bUartMode=%d\n",
    __func__, baud, wbaudrate, wflags,
    config.bDataBits, config.bParity, config.bStopBits,
    config.cXon, config.cXoff, config.bUartMode);
    config.wBaudRate = cpu_to_be16(wbaudrate);
    config.wFlags = cpu_to_be16(wflags);
    status = ti_port_cmd_out(port, TI_SET_CONFIG, 0, config,
    sizeof(*config));
    if (status)
    dev_err(&port.dev, "%s - cannot set config on port %d, %d\n",
    __func__, port.port_number, status);
// SET_CONFIG asserts RTS and DTR, reset them correctly
    mcr = tport.tp_shadow_mcr;
// if baud rate is B0, clear RTS and DTR
    if (C_BAUD(tty) == B0)
    mcr &= ~(TI_MCR_DTR | TI_MCR_RTS);
    status = ti_set_mcr(tport, mcr);
    if (status)
    dev_err(&port.dev, "%s - cannot set modem control on port %d, %d\n",
    __func__, port.port_number, status);
    kfree(config);
    }
#[no_mangle]
unsafe extern "C" fn ti_tiocmget(tty: *mut tty_struct) -> c_int {
    static int ti_tiocmget(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct ti_port *tport = usb_get_serial_port_data(port);
    unsigned int result;
    unsigned int msr;
    unsigned int mcr;
    unsigned long flags;
    spin_lock_irqsave(&tport.tp_lock, flags);
    msr = tport.tp_msr;
    mcr = tport.tp_shadow_mcr;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    result = ((mcr & TI_MCR_DTR) ? TIOCM_DTR : 0)
    | ((mcr & TI_MCR_RTS) ? TIOCM_RTS : 0)
    | ((mcr & TI_MCR_LOOP) ? TIOCM_LOOP : 0)
    | ((msr & TI_MSR_CTS) ? TIOCM_CTS : 0)
    | ((msr & TI_MSR_CD) ? TIOCM_CAR : 0)
    | ((msr & TI_MSR_RI) ? TIOCM_RI : 0)
    | ((msr & TI_MSR_DSR) ? TIOCM_DSR : 0);
    dev_dbg(&port.dev, "%s - 0x%04X\n", __func__, result);
    return result;
    }
    static int ti_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct ti_port *tport = usb_get_serial_port_data(port);
    unsigned int mcr;
    unsigned long flags;
    spin_lock_irqsave(&tport.tp_lock, flags);
    mcr = tport.tp_shadow_mcr;
    if (set & TIOCM_RTS)
    mcr |= TI_MCR_RTS;
    if (set & TIOCM_DTR)
    mcr |= TI_MCR_DTR;
    if (set & TIOCM_LOOP)
    mcr |= TI_MCR_LOOP;
    if (clear & TIOCM_RTS)
    mcr &= ~TI_MCR_RTS;
    if (clear & TIOCM_DTR)
    mcr &= ~TI_MCR_DTR;
    if (clear & TIOCM_LOOP)
    mcr &= ~TI_MCR_LOOP;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    return ti_set_mcr(tport, mcr);
    }
#[no_mangle]
unsafe extern "C" fn ti_break(tty: *mut tty_struct, break_state: c_int) -> c_int {
    static int ti_break(struct tty_struct *tty, int break_state)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct ti_port *tport = usb_get_serial_port_data(port);
    int status;
    dev_dbg(&port.dev, "%s - state = %d\n", __func__, break_state);
    status = ti_write_byte(port, tport.tp_tdev,
    tport.tp_uart_base_addr + TI_UART_OFFSET_LCR,
    TI_LCR_BREAK, break_state == -1 ? TI_LCR_BREAK : 0);
    if (status) {
    dev_dbg(&port.dev, "%s - error setting break, %d\n", __func__, status);
    return status;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_get_port_from_code(code: c_uchar) -> c_int {
    static int ti_get_port_from_code(unsigned char code)
    {
    return (code >> 6) & 0x01;
    }
#[no_mangle]
unsafe extern "C" fn ti_get_func_from_code(code: c_uchar) -> c_int {
    static int ti_get_func_from_code(unsigned char code)
    {
    return code & 0x0f;
    }
#[no_mangle]
unsafe extern "C" fn ti_interrupt_callback(urb: *mut urb) {
    static void ti_interrupt_callback(struct urb *urb)
    {
    struct ti_device *tdev = urb.context;
    struct usb_serial_port *port;
    struct usb_serial *serial = tdev.td_serial;
    struct ti_port *tport;
    struct device *dev = &urb.dev.dev;
    unsigned char *data = urb.transfer_buffer;
    let mut length: c_int = urb.actual_length;
    int port_number;
    int function;
    let mut status: c_int = urb.status;
    int retval;
    u8 msr;
    switch (status) {
    case 0:
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    dev_dbg(dev, "%s - urb shutting down, %d\n", __func__, status);
    return;
    default:
    dev_err(dev, "%s - nonzero urb status, %d\n", __func__, status);
    goto exit;
    }
    if (length != 2) {
    dev_dbg(dev, "%s - bad packet size, %d\n", __func__, length);
    goto exit;
    }
    if (data[0] == TI_CODE_HARDWARE_ERROR) {
    dev_err(dev, "%s - hardware error, %d\n", __func__, data[1]);
    goto exit;
    }
    port_number = ti_get_port_from_code(data[0]);
    function = ti_get_func_from_code(data[0]);
    dev_dbg(dev, "%s - port_number %d, function %d, data 0x%02X\n",
    __func__, port_number, function, data[1]);
    if (port_number >= serial.num_ports) {
    dev_err(dev, "%s - bad port number, %d\n",
    __func__, port_number);
    goto exit;
    }
    port = serial.port[port_number];
    tport = usb_get_serial_port_data(port);
    if (!tport)
    goto exit;
    switch (function) {
    case TI_CODE_DATA_ERROR:
    dev_err(dev, "%s - DATA ERROR, port %d, data 0x%02X\n",
    __func__, port_number, data[1]);
    break;
    case TI_CODE_MODEM_STATUS:
    msr = data[1];
    dev_dbg(dev, "%s - port %d, msr 0x%02X\n", __func__, port_number, msr);
    ti_handle_new_msr(tport, msr);
    break;
    default:
    dev_err(dev, "%s - unknown interrupt code, 0x%02X\n",
    __func__, data[1]);
    break;
    }
    exit:
    retval = usb_submit_urb(urb, GFP_ATOMIC);
    if (retval)
    dev_err(dev, "%s - resubmit interrupt urb failed, %d\n",
    __func__, retval);
    }
#[no_mangle]
unsafe extern "C" fn ti_bulk_in_callback(urb: *mut urb) {
    static void ti_bulk_in_callback(struct urb *urb)
    {
    struct ti_port *tport = urb.context;
    struct usb_serial_port *port = tport.tp_port;
    struct device *dev = &urb.dev.dev;
    let mut status: c_int = urb.status;
    unsigned long flags;
    let mut retval: c_int = 0;
    switch (status) {
    case 0:
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    dev_dbg(dev, "%s - urb shutting down, %d\n", __func__, status);
    return;
    default:
    dev_err(dev, "%s - nonzero urb status, %d\n",
    __func__, status);
    }
    if (status == -EPIPE)
    goto exit;
    if (status) {
    dev_err(dev, "%s - stopping read!\n", __func__);
    return;
    }
    if (urb.actual_length) {
    usb_serial_debug_data(dev, __func__, urb.actual_length,
    urb.transfer_buffer);
    if (!tport.tp_is_open)
    dev_dbg(dev, "%s - port closed, dropping data\n",
    __func__);
    else
    ti_recv(port, urb.transfer_buffer, urb.actual_length);
    spin_lock_irqsave(&tport.tp_lock, flags);
    port.icount.rx += urb.actual_length;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    }
    exit:
// continue to read unless stopping
    spin_lock_irqsave(&tport.tp_lock, flags);
    if (tport.tp_read_urb_state == TI_READ_URB_RUNNING)
    retval = usb_submit_urb(urb, GFP_ATOMIC);
#[no_mangle]
pub unsafe extern "C" fn if(TI_READ_URB_STOPPING: tport->tp_read_urb_state ==) -> else {
    else if (tport.tp_read_urb_state == TI_READ_URB_STOPPING)
    tport.tp_read_urb_state = TI_READ_URB_STOPPED;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    if (retval)
    dev_err(dev, "%s - resubmit read urb failed, %d\n",
    __func__, retval);
    }
#[no_mangle]
unsafe extern "C" fn ti_bulk_out_callback(urb: *mut urb) {
    static void ti_bulk_out_callback(struct urb *urb)
    {
    struct ti_port *tport = urb.context;
    struct usb_serial_port *port = tport.tp_port;
    let mut status: c_int = urb.status;
    tport.tp_write_urb_in_use = 0;
    switch (status) {
    case 0:
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    dev_dbg(&port.dev, "%s - urb shutting down, %d\n", __func__, status);
    return;
    default:
    dev_err_console(port, "%s - nonzero urb status, %d\n",
    __func__, status);
    }
// send any buffered data
    ti_send(tport);
    }
    static void ti_recv(struct usb_serial_port *port, unsigned char *data,
    int length)
    {
    int cnt;
    do {
    cnt = tty_insert_flip_string(&port.port, data, length);
    if (cnt < length) {
    dev_err(&port.dev, "%s - dropping data, %d bytes lost\n",
    __func__, length - cnt);
    if (cnt == 0)
    break;
    }
    tty_flip_buffer_push(&port.port);
    data += cnt;
    length -= cnt;
    } while (length > 0);
    }
#[no_mangle]
unsafe extern "C" fn ti_send(tport: *mut ti_port) {
    static void ti_send(struct ti_port *tport)
    {
    int count, result;
    struct usb_serial_port *port = tport.tp_port;
    unsigned long flags;
    spin_lock_irqsave(&tport.tp_lock, flags);
    if (tport.tp_write_urb_in_use)
    goto unlock;
    count = kfifo_out(&port.write_fifo,
    port.write_urb.transfer_buffer,
    port.bulk_out_size);
    if (count == 0)
    goto unlock;
    tport.tp_write_urb_in_use = 1;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    usb_serial_debug_data(&port.dev, __func__, count,
    port.write_urb.transfer_buffer);
    usb_fill_bulk_urb(port.write_urb, port.serial.dev,
    usb_sndbulkpipe(port.serial.dev,
    port.bulk_out_endpointAddress),
    port.write_urb.transfer_buffer, count,
    ti_bulk_out_callback, tport);
    result = usb_submit_urb(port.write_urb, GFP_ATOMIC);
    if (result) {
    dev_err_console(port, "%s - submit write urb failed, %d\n",
    __func__, result);
    tport.tp_write_urb_in_use = 0;
// TODO: reschedule ti_send
    } else {
    spin_lock_irqsave(&tport.tp_lock, flags);
    port.icount.tx += count;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    }
// more room in the buffer for new writes, wakeup
    tty_port_tty_wakeup(&port.port);
    return;
    unlock:
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    return;
    }
#[no_mangle]
unsafe extern "C" fn ti_set_mcr(tport: *mut ti_port, mcr: c_uint) -> c_int {
    static int ti_set_mcr(struct ti_port *tport, unsigned int mcr)
    {
    unsigned long flags;
    int status;
    status = ti_write_byte(tport.tp_port, tport.tp_tdev,
    tport.tp_uart_base_addr + TI_UART_OFFSET_MCR,
    TI_MCR_RTS | TI_MCR_DTR | TI_MCR_LOOP, mcr);
    spin_lock_irqsave(&tport.tp_lock, flags);
    if (!status)
    tport.tp_shadow_mcr = mcr;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ti_get_lsr(tport: *mut ti_port, lsr: *mut u8) -> c_int {
    static int ti_get_lsr(struct ti_port *tport, u8 *lsr)
    {
    int size, status;
    struct usb_serial_port *port = tport.tp_port;
    struct ti_port_status *data;
    size = sizeof(struct ti_port_status);
    data = kmalloc(size, GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    status = ti_port_cmd_in(port, TI_GET_PORT_STATUS, 0, data, size);
    if (status) {
    dev_err(&port.dev,
    "%s - get port status command failed, %d\n",
    __func__, status);
    goto free_data;
    }
    dev_dbg(&port.dev, "%s - lsr 0x%02X\n", __func__, data.bLSR);
// lsr = data->bLSR;
    free_data:
    kfree(data);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ti_get_serial_info(tty: *mut tty_struct, ss: *mut serial_struct) {
    static void ti_get_serial_info(struct tty_struct *tty, struct serial_struct *ss)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct ti_port *tport = usb_get_serial_port_data(port);
    ss.baud_base = tport.tp_tdev.td_is_3410 ? 921600 : 460800;
    }
#[no_mangle]
unsafe extern "C" fn ti_handle_new_msr(tport: *mut ti_port, msr: u8) {
    static void ti_handle_new_msr(struct ti_port *tport, u8 msr)
    {
    struct async_icount *icount;
    struct tty_struct *tty;
    unsigned long flags;
    dev_dbg(&tport.tp_port.dev, "%s - msr 0x%02X\n", __func__, msr);
    if (msr & TI_MSR_DELTA_MASK) {
    spin_lock_irqsave(&tport.tp_lock, flags);
    icount = &tport.tp_port.icount;
    if (msr & TI_MSR_DELTA_CTS)
    icount.cts++;
    if (msr & TI_MSR_DELTA_DSR)
    icount.dsr++;
    if (msr & TI_MSR_DELTA_CD)
    icount.dcd++;
    if (msr & TI_MSR_DELTA_RI)
    icount.rng++;
    wake_up_interruptible(&tport.tp_port.port.delta_msr_wait);
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    }
    tport.tp_msr = msr & TI_MSR_MASK;
// handle CTS flow control
    tty = tty_port_tty_get(&tport.tp_port.port);
    if (tty && C_CRTSCTS(tty)) {
    if (msr & TI_MSR_CTS)
    tty_wakeup(tty);
    }
    tty_kref_put(tty);
    }
#[no_mangle]
unsafe extern "C" fn ti_stop_read(tport: *mut ti_port, tty: *mut tty_struct) {
    static void ti_stop_read(struct ti_port *tport, struct tty_struct *tty)
    {
    unsigned long flags;
    spin_lock_irqsave(&tport.tp_lock, flags);
    if (tport.tp_read_urb_state == TI_READ_URB_RUNNING)
    tport.tp_read_urb_state = TI_READ_URB_STOPPING;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ti_restart_read(tport: *mut ti_port, tty: *mut tty_struct) -> c_int {
    static int ti_restart_read(struct ti_port *tport, struct tty_struct *tty)
    {
    struct urb *urb;
    let mut status: c_int = 0;
    unsigned long flags;
    spin_lock_irqsave(&tport.tp_lock, flags);
    if (tport.tp_read_urb_state == TI_READ_URB_STOPPED) {
    tport.tp_read_urb_state = TI_READ_URB_RUNNING;
    urb = tport.tp_port.read_urb;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    urb.context = tport;
    status = usb_submit_urb(urb, GFP_KERNEL);
    } else  {
    tport.tp_read_urb_state = TI_READ_URB_RUNNING;
    spin_unlock_irqrestore(&tport.tp_lock, flags);
    }
    return status;
    }
    static int ti_command_out_sync(struct usb_device *udev, u8 command,
    u16 moduleid, u16 value, void *data, int size)
    {
    int status;
    status = usb_control_msg(udev, usb_sndctrlpipe(udev, 0), command,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT,
    value, moduleid, data, size, 1000);
    if (status < 0)
    return status;
    return 0;
    }
    static int ti_command_in_sync(struct usb_device *udev, u8 command,
    u16 moduleid, u16 value, void *data, int size)
    {
    int status;
    status = usb_control_msg(udev, usb_rcvctrlpipe(udev, 0), command,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    value, moduleid, data, size, 1000);
    if (status == size)
    status = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: status >=) -> else {
    else if (status >= 0)
    status = -ECOMM;
    return status;
    }
    static int ti_port_cmd_out(struct usb_serial_port *port, u8 command,
    u16 value, void *data, int size)
    {
    return ti_command_out_sync(port.serial.dev, command,
    TI_UART1_PORT + port.port_number,
    value, data, size);
    }
    static int ti_port_cmd_in(struct usb_serial_port *port, u8 command,
    u16 value, void *data, int size)
    {
    return ti_command_in_sync(port.serial.dev, command,
    TI_UART1_PORT + port.port_number,
    value, data, size);
    }
    static int ti_write_byte(struct usb_serial_port *port,
    struct ti_device *tdev, unsigned long addr,
    u8 mask, u8 byte)
    {
    int status;
    unsigned int size;
    struct ti_write_data_bytes *data;
    dev_dbg(&port.dev, "%s - addr 0x%08lX, mask 0x%02X, byte 0x%02X\n", __func__,
    addr, mask, byte);
    size = sizeof(struct ti_write_data_bytes) + 2;
    data = kmalloc(size, GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.bAddrType = TI_RW_DATA_ADDR_XDATA;
    data.bDataType = TI_RW_DATA_BYTE;
    data.bDataCounter = 1;
    data.wBaseAddrHi = cpu_to_be16(addr>>16);
    data.wBaseAddrLo = cpu_to_be16(addr);
    data.bData[0] = mask;
    data.bData[1] = byte;
    status = ti_command_out_sync(port.serial.dev, TI_WRITE_DATA,
    TI_RAM_PORT, 0, data, size);
    if (status < 0)
    dev_err(&port.dev, "%s - failed, %d\n", __func__, status);
    kfree(data);
    return status;
    }
    static int ti_do_download(struct usb_device *dev, int pipe,
    u8 *buffer, int size)
    {
    int pos;
    let mut cs: u8 = 0;
    int done;
    struct ti_firmware_header *header;
    let mut status: c_int = 0;
    int len;
    for (pos = sizeof(struct ti_firmware_header); pos < size; pos++)
    cs = (u8)(cs + buffer[pos]);
    header = (struct ti_firmware_header *)buffer;
    header.wLength = cpu_to_le16(size - sizeof(*header));
    header.bCheckSum = cs;
    dev_dbg(&dev.dev, "%s - downloading firmware\n", __func__);
    for (pos = 0; pos < size; pos += done) {
    len = min(size - pos, TI_DOWNLOAD_MAX_PACKET_SIZE);
    status = usb_bulk_msg(dev, pipe, buffer + pos, len,
    &done, 1000);
    if (status)
    break;
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ti_download_firmware(tdev: *mut ti_device) -> c_int {
    static int ti_download_firmware(struct ti_device *tdev)
    {
    int status;
    int buffer_size;
    u8 *buffer;
    struct usb_device *dev = tdev.td_serial.dev;
    unsigned int pipe = usb_sndbulkpipe(dev,
    tdev.td_serial.port[0].bulk_out_endpointAddress);
    const struct firmware *fw_p;
    char buf[32];
    if (le16_to_cpu(dev.descriptor.idVendor) == MXU1_VENDOR_ID) {
    snprintf(buf,
    sizeof(buf),
    "moxa/moxa-%04x.fw",
    le16_to_cpu(dev.descriptor.idProduct));
    status = request_firmware(&fw_p, buf, &dev.dev);
    goto check_firmware;
    }
// try ID specific firmware first, then try generic firmware
    sprintf(buf, "ti_usb-v%04x-p%04x.fw",
    le16_to_cpu(dev.descriptor.idVendor),
    le16_to_cpu(dev.descriptor.idProduct));
    status = request_firmware(&fw_p, buf, &dev.dev);
    if (status != 0) {
    buf[0] = '\0';
    if (le16_to_cpu(dev.descriptor.idVendor) == MTS_VENDOR_ID) {
    switch (le16_to_cpu(dev.descriptor.idProduct)) {
    case MTS_CDMA_PRODUCT_ID:
    strscpy(buf, "mts_cdma.fw");
    break;
    case MTS_GSM_PRODUCT_ID:
    strscpy(buf, "mts_gsm.fw");
    break;
    case MTS_EDGE_PRODUCT_ID:
    strscpy(buf, "mts_edge.fw");
    break;
    case MTS_MT9234MU_PRODUCT_ID:
    strscpy(buf, "mts_mt9234mu.fw");
    break;
    case MTS_MT9234ZBA_PRODUCT_ID:
    strscpy(buf, "mts_mt9234zba.fw");
    break;
    case MTS_MT9234ZBAOLD_PRODUCT_ID:
    strscpy(buf, "mts_mt9234zba.fw");
    break;			}
    }
    if (buf[0] == '\0') {
    if (tdev.td_is_3410)
    strscpy(buf, "ti_3410.fw");
    else
    strscpy(buf, "ti_5052.fw");
    }
    status = request_firmware(&fw_p, buf, &dev.dev);
    }
    check_firmware:
    if (status) {
    dev_err(&dev.dev, "%s - firmware not found\n", __func__);
    return -ENOENT;
    }
    if (fw_p.size > TI_FIRMWARE_BUF_SIZE) {
    dev_err(&dev.dev, "%s - firmware too large %zu\n", __func__, fw_p.size);
    release_firmware(fw_p);
    return -ENOENT;
    }
    buffer_size = TI_FIRMWARE_BUF_SIZE + sizeof(struct ti_firmware_header);
    buffer = kmalloc(buffer_size, GFP_KERNEL);
    if (buffer) {
    memcpy(buffer, fw_p.data, fw_p.size);
    memset(buffer + fw_p.size, 0xff, buffer_size - fw_p.size);
    status = ti_do_download(dev, pipe, buffer, fw_p.size);
    kfree(buffer);
    } else {
    status = -ENOMEM;
    }
    release_firmware(fw_p);
    if (status) {
    dev_err(&dev.dev, "%s - error downloading firmware, %d\n",
    __func__, status);
    return status;
    }
    dev_dbg(&dev.dev, "%s - download successful\n", __func__);
    return 0;
    }
