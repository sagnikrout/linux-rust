//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_men_mcb.c
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

pub const MEN_UART_ID_Z025: c_uint = 0x19;
pub const MEN_UART_ID_Z057: c_uint = 0x39;
pub const MEN_UART_ID_Z125: c_uint = 0x7d;
//
// IP Cores Z025 and Z057 can have up to 4 UART
// The UARTs available are stored in a global
// register saved in physical address + 0x40
// Is saved as follows:
//
// 7                                                              0
// +------+-------+-------+-------+-------+-------+-------+-------+
// |UART4 | UART3 | UART2 | UART1 | U4irq | U3irq | U2irq | U1irq |
// +------+-------+-------+-------+-------+-------+-------+-------+
//
pub const MEN_UART1_MASK: c_uint = 0x01;
pub const MEN_UART2_MASK: c_uint = 0x02;
pub const MEN_UART3_MASK: c_uint = 0x04;
pub const MEN_UART4_MASK: c_uint = 0x08;
pub const MEN_Z025_MAX_UARTS: c_int = 4;
pub const MEN_UART_MEM_SIZE: c_uint = 0x10;
pub const MEM_UART_REGISTER_SIZE: c_uint = 0x01;
pub const MEN_Z025_REGISTER_OFFSET: c_uint = 0x40;
pub const MEN_UART1_OFFSET: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_8250_men_mcb_data {
    pub num_ports: c_int,
    pub line: [c_int; MEN_Z025_MAX_UARTS],
    pub offset: [c_uint; MEN_Z025_MAX_UARTS],
}

//
// The Z125 16550-compatible UART has no fixed base clock assigned
// So, depending on the board we're on, we need to adjust the
// parameter in order to really set the correct baudrate, and
// do so if possible without user interaction
//
#[no_mangle]
unsafe extern "C" fn men_lookup_uartclk(mdev: *mut mcb_device) -> u32 {
    static u32 men_lookup_uartclk(struct mcb_device *mdev)
    {
// use default value if board is not available below
    let mut clkval: u32 = 1041666;
    dev_info(&mdev.dev, "%s on board %s\n",
    dev_name(&mdev.dev),
    mdev.bus.name);
    if  (strncmp(mdev.bus.name, "F075", 4) == 0)
    clkval = 1041666;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strncmp(mdev->bus->name, _arg: "F216", 0: 4) ==) -> else {
    else if (strncmp(mdev.bus.name, "F216", 4) == 0)
    clkval = 1843200;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strncmp(mdev->bus->name, _arg: "F210", 0: 4) ==) -> else {
    else if (strncmp(mdev.bus.name, "F210", 4) == 0)
    clkval = 115200;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(mdev->bus->name, _arg: "215")) -> else {
    else if (strstr(mdev.bus.name, "215"))
    clkval = 1843200;
    else
    dev_info(&mdev.dev,
    "board not detected, using default uartclk\n");
    clkval = clkval  << 4;
    return clkval;
    }
    static int read_uarts_available_from_register(struct resource *mem_res,
    u8 *uarts_available)
    {
    void __iomem *mem;
    int reg_value;
    if (!request_mem_region(mem_res.start + MEN_Z025_REGISTER_OFFSET,
    MEM_UART_REGISTER_SIZE,  KBUILD_MODNAME)) {
    return -EBUSY;
    }
    mem = ioremap(mem_res.start + MEN_Z025_REGISTER_OFFSET,
    MEM_UART_REGISTER_SIZE);
    if (!mem) {
    release_mem_region(mem_res.start + MEN_Z025_REGISTER_OFFSET,
    MEM_UART_REGISTER_SIZE);
    return -ENOMEM;
    }
    reg_value = MEN_READ_REGISTER(mem);
    iounmap(mem);
    release_mem_region(mem_res.start + MEN_Z025_REGISTER_OFFSET,
    MEM_UART_REGISTER_SIZE);
// uarts_available = reg_value >> 4;
    return 0;
    }
    static int read_serial_data(struct mcb_device *mdev,
    struct resource *mem_res,
    struct serial_8250_men_mcb_data *serial_data)
    {
    u8 uarts_available;
    let mut count: c_int = 0;
    int mask;
    int res;
    int i;
    res = read_uarts_available_from_register(mem_res, &uarts_available);
    if (res < 0)
    return res;
    for (i = 0; i < MEN_Z025_MAX_UARTS; i++) {
    mask = 0x1 << i;
    switch (uarts_available & mask) {
    case MEN_UART1_MASK:
    serial_data.offset[count] = MEN_UART1_OFFSET;
    count++;
    break;
    case MEN_UART2_MASK:
    serial_data.offset[count] = MEN_UART2_OFFSET;
    count++;
    break;
    case MEN_UART3_MASK:
    serial_data.offset[count] = MEN_UART3_OFFSET;
    count++;
    break;
    case MEN_UART4_MASK:
    serial_data.offset[count] = MEN_UART4_OFFSET;
    count++;
    break;
    default:
    return -EINVAL;
    }
    }
    if (count <= 0 || count > MEN_Z025_MAX_UARTS) {
    dev_err(&mdev.dev, "unexpected number of ports: %u\n",
    count);
    return -ENODEV;
    }
    serial_data.num_ports = count;
    return 0;
    }
    static int init_serial_data(struct mcb_device *mdev,
    struct resource *mem_res,
    struct serial_8250_men_mcb_data *serial_data)
    {
    switch (mdev.id) {
    case MEN_UART_ID_Z125:
    serial_data.num_ports = 1;
    serial_data.offset[0] = 0;
    return 0;
    case MEN_UART_ID_Z025:
    case MEN_UART_ID_Z057:
    return read_serial_data(mdev, mem_res, serial_data);
    default:
    dev_err(&mdev.dev, "no supported device!\n");
    return -ENODEV;
    }
    }
    static int serial_8250_men_mcb_probe(struct mcb_device *mdev,
    const struct mcb_device_id *id)
    {
    struct uart_8250_port uart;
    struct serial_8250_men_mcb_data *data;
    struct resource *mem;
    int i;
    int res;
    mem = mcb_get_resource(mdev, IORESOURCE_MEM);
    if (mem == core::ptr::null_mut())
    return -ENXIO;
    data = devm_kzalloc(&mdev.dev,
    sizeof(struct serial_8250_men_mcb_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    res = init_serial_data(mdev, mem, data);
    if (res < 0)
    return res;
    dev_dbg(&mdev.dev, "found a 16z%03u with %u ports\n",
    mdev.id, data.num_ports);
    mcb_set_drvdata(mdev, data);
    for (i = 0; i < data.num_ports; i++) {
    memset(&uart, 0, sizeof(struct uart_8250_port));
    spin_lock_init(&uart.port.lock);
    uart.port.flags = UPF_SKIP_TEST |
    UPF_SHARE_IRQ |
    UPF_BOOT_AUTOCONF |
    UPF_IOREMAP;
    uart.port.iotype = UPIO_MEM;
    uart.port.uartclk = men_lookup_uartclk(mdev);
    uart.port.irq = mcb_get_irq(mdev);
    uart.port.mapbase = (unsigned long) mem.start
    + data.offset[i];
// ok, register the port
    res = serial8250_register_8250_port(&uart);
    if (res < 0) {
    dev_err(&mdev.dev, "unable to register UART port\n");
    return res;
    }
    data.line[i] = res;
    dev_info(&mdev.dev, "found MCB UART: ttyS%d\n", data.line[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial_8250_men_mcb_remove(mdev: *mut mcb_device) {
    static void serial_8250_men_mcb_remove(struct mcb_device *mdev)
    {
    int i;
    struct serial_8250_men_mcb_data *data = mcb_get_drvdata(mdev);
    if (!data)
    return;
    for (i = 0; i < data.num_ports; i++)
    serial8250_unregister_port(data.line[i]);
    }
    static const struct mcb_device_id serial_8250_men_mcb_ids[] = {
    { .device = MEN_UART_ID_Z025 },
    { .device = MEN_UART_ID_Z057 },
    { .device = MEN_UART_ID_Z125 },
    { }
    };
    MODULE_DEVICE_TABLE(mcb, serial_8250_men_mcb_ids);
    static struct mcb_driver mcb_driver = {
    .driver = {
    .name = "8250_men_mcb",
    },
    .probe = serial_8250_men_mcb_probe,
    .remove = serial_8250_men_mcb_remove,
    .id_table = serial_8250_men_mcb_ids,
    };
    module_mcb_driver(mcb_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("MEN 8250 UART driver");
    MODULE_AUTHOR("Michael Moese <michael.moese@men.de");
    MODULE_IMPORT_NS("MCB");
