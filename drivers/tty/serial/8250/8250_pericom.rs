//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_pericom.c
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
// Driver for Pericom UART

pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM_2SDB: c_uint = 0x1051;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_COM_2S: c_uint = 0x1053;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM422_4: c_uint = 0x105a;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM485_4: c_uint = 0x105b;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM_4SDB: c_uint = 0x105c;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_COM_4S: c_uint = 0x105e;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM422_8: c_uint = 0x106a;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM485_8: c_uint = 0x106b;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM232_2DB: c_uint = 0x1091;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_COM232_2: c_uint = 0x1093;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM232_4: c_uint = 0x1098;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM232_4DB: c_uint = 0x1099;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_COM232_4: c_uint = 0x109b;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM232_8: c_uint = 0x10a9;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM_2SMDB: c_uint = 0x10d1;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_COM_2SM: c_uint = 0x10d3;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM_4SM: c_uint = 0x10d9;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM_4SMDB: c_uint = 0x10da;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_COM_4SM: c_uint = 0x10dc;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_COM_8SM: c_uint = 0x10e9;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM485_1: c_uint = 0x1108;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM422_2: c_uint = 0x1110;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM485_2: c_uint = 0x1111;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM422_4: c_uint = 0x1118;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM485_4: c_uint = 0x1119;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_ICM_2S: c_uint = 0x1152;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_ICM_4S: c_uint = 0x115a;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_ICM232_2: c_uint = 0x1190;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM232_2: c_uint = 0x1191;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_ICM232_4: c_uint = 0x1198;
pub const PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM232_4: c_uint = 0x1199;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_ICM_2SM: c_uint = 0x11d0;
pub const PCI_DEVICE_ID_ACCESSIO_PCIE_ICM_4SM: c_uint = 0x11d8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pericom8250 {
    pub virt: *mut void __iomem,
    pub nr: c_uint,
    pub line: [c_int; ],
}

    static void pericom_do_set_divisor(struct uart_port *port, unsigned int baud,
    unsigned int quot, unsigned int quot_frac)
    {
    int scr;
    for (scr = 16; scr > 4; scr--) {
    let mut maxrate: c_uint = port.uartclk / scr;
    let mut divisor: c_uint = max(maxrate / baud, 1U);
    let mut delta: c_int = maxrate / divisor - baud;
    if (baud > maxrate + baud / 50)
    continue;
    if (delta > baud / 50)
    divisor++;
    if (divisor > 0xffff)
    continue;
// Update delta due to possible divisor change
    delta = maxrate / divisor - baud;
    if (abs(delta) < baud / 50) {
    struct uart_8250_port *up = up_to_u8250p(port);
    let mut lcr: c_int = serial_port_in(port, UART_LCR);
    serial_port_out(port, UART_LCR, lcr | UART_LCR_DLAB);
    serial_dl_write(up, divisor);
    serial_port_out(port, 2, 16 - scr);
    serial_port_out(port, UART_LCR, lcr);
    return;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pericom8250_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int pericom8250_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    unsigned int nr, i, bar = 0, maxnr;
    struct pericom8250 *pericom;
    struct uart_8250_port uart;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    maxnr = pci_resource_len(pdev, bar) >> 3;
    if (pdev.vendor == PCI_VENDOR_ID_PERICOM)
    nr = pdev.device & 0x0f;
#[no_mangle]
pub unsafe extern "C" fn if(PCI_VENDOR_ID_ACCESSIO: pdev->vendor ==) -> else {
    else if (pdev.vendor == PCI_VENDOR_ID_ACCESSIO)
    nr = BIT(((pdev.device & 0x38) >> 3) - 1);
    else
    nr = 1;
    pericom = devm_kzalloc(&pdev.dev, struct_size(pericom, line, nr), GFP_KERNEL);
    if (!pericom)
    return -ENOMEM;
    pericom.virt = pcim_iomap(pdev, bar, 0);
    if (!pericom.virt)
    return -ENOMEM;
    memset(&uart, 0, sizeof(uart));
    uart.port.dev = &pdev.dev;
    uart.port.irq = pdev.irq;
    uart.port.private_data = pericom;
    uart.port.iotype = UPIO_PORT;
    uart.port.uartclk = 921600 * 16;
    uart.port.flags = UPF_SKIP_TEST | UPF_BOOT_AUTOCONF | UPF_SHARE_IRQ;
    uart.port.set_divisor = pericom_do_set_divisor;
    for (i = 0; i < nr && i < maxnr; i++) {
    let mut offset: c_uint = (i == 3 && nr == 4) ? 0x38 : i * 0x8;
    uart.port.iobase = pci_resource_start(pdev, bar) + offset;
    dev_dbg(&pdev.dev, "Setup PCI port: port %lx, irq %d, type %d\n",
    uart.port.iobase, uart.port.irq, uart.port.iotype);
    pericom.line[i] = serial8250_register_8250_port(&uart);
    if (pericom.line[i] < 0) {
    dev_err(&pdev.dev,
    "Couldn't register serial port %lx, irq %d, type %d, error %d\n",
    uart.port.iobase, uart.port.irq,
    uart.port.iotype, pericom.line[i]);
    break;
    }
    }
    pericom.nr = i;
    pci_set_drvdata(pdev, pericom);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pericom8250_remove(pdev: *mut pci_dev) {
    static void pericom8250_remove(struct pci_dev *pdev)
    {
    struct pericom8250 *pericom = pci_get_drvdata(pdev);
    unsigned int i;
    for (i = 0; i < pericom.nr; i++)
    serial8250_unregister_port(pericom.line[i]);
    }
    static const struct pci_device_id pericom8250_pci_ids[] = {
//
// Pericom PI7C9X795[1248] Uno/Dual/Quad/Octal UART
// (Only 7954 has an offset jump for port 4)
//
    { PCI_VDEVICE(PERICOM, PCI_DEVICE_ID_PERICOM_PI7C9X7951) },
    { PCI_VDEVICE(PERICOM, PCI_DEVICE_ID_PERICOM_PI7C9X7952) },
    { PCI_VDEVICE(PERICOM, PCI_DEVICE_ID_PERICOM_PI7C9X7954) },
    { PCI_VDEVICE(PERICOM, PCI_DEVICE_ID_PERICOM_PI7C9X7958) },
//
// ACCES I/O Products quad
// (Only 7954 has an offset jump for port 4)
//
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM_2SDB) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_COM_2S) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM422_4) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM485_4) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM_4SDB) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_COM_4S) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM422_8) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM485_8) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM232_2DB) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_COM232_2) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM232_4) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM232_4DB) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_COM232_4) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM232_8) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM_2SMDB) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_COM_2SM) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM_4SM) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM_4SMDB) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_COM_4SM) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_COM_8SM) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM485_1) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM422_2) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM485_2) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM422_4) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM485_4) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_ICM_2S) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_ICM_4S) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_ICM232_2) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM232_2) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_ICM232_4) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_MPCIE_ICM232_4) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_ICM_2SM) },
    { PCI_VDEVICE(ACCESSIO, PCI_DEVICE_ID_ACCESSIO_PCIE_ICM_4SM) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pericom8250_pci_ids);
    static struct pci_driver pericom8250_pci_driver = {
    .name           = "8250_pericom",
    .id_table       = pericom8250_pci_ids,
    .probe          = pericom8250_probe,
    .remove         = pericom8250_remove,
    };
    module_pci_driver(pericom8250_pci_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Pericom UART driver");
