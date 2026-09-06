//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/tegra-tcu.c
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
//
// Copyright (c) 2018, NVIDIA CORPORATION.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_tcu {
    pub driver: uart_driver,

    pub console: console,

    pub port: uart_port,
    pub rx_client: mbox_client tx_client,,
    pub rx: *mut *mut mbox_chan tx,,
}

#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int tegra_tcu_uart_tx_empty(struct uart_port *port)
    {
    return TIOCSER_TEMT;
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void tegra_tcu_uart_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int tegra_tcu_uart_get_mctrl(struct uart_port *port)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_stop_tx(port: *mut uart_port) {
    static void tegra_tcu_uart_stop_tx(struct uart_port *port)
    {
    }
    static void tegra_tcu_write_one(struct tegra_tcu *tcu, u32 value,
    unsigned int count)
    {
    void *msg;
    value |= TCU_MBOX_NUM_BYTES(count);
    msg = (void *)(unsigned long)value;
    mbox_send_message(tcu.tx, msg);
    mbox_flush(tcu.tx, 1000);
    }
    static void tegra_tcu_write(struct tegra_tcu *tcu, const char *s,
    unsigned int count)
    {
    let mut written: c_uint = 0, i = 0;
    let mut insert_nl: bool = false;
    let mut value: u32 = 0;
    while (i < count) {
    if (insert_nl) {
    value |= TCU_MBOX_BYTE(written++, '\n');
    insert_nl = false;
    i++;
    } else if (s[i] == '\n') {
    value |= TCU_MBOX_BYTE(written++, '\r');
    insert_nl = true;
    } else {
    value |= TCU_MBOX_BYTE(written++, s[i++]);
    }
    if (written == 3) {
    tegra_tcu_write_one(tcu, value, 3);
    value = written = 0;
    }
    }
    if (written)
    tegra_tcu_write_one(tcu, value, written);
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_start_tx(port: *mut uart_port) {
    static void tegra_tcu_uart_start_tx(struct uart_port *port)
    {
    struct tegra_tcu *tcu = port.private_data;
    struct tty_port *tport = &port.state.port;
    unsigned char *tail;
    unsigned int count;
    for (;;) {
    count = kfifo_out_linear_ptr(&tport.xmit_fifo, &tail,
    UART_XMIT_SIZE);
    if (!count)
    break;
    tegra_tcu_write(tcu, tail, count);
    uart_xmit_advance(port, count);
    }
    uart_write_wakeup(port);
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_stop_rx(port: *mut uart_port) {
    static void tegra_tcu_uart_stop_rx(struct uart_port *port)
    {
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_break_ctl(port: *mut uart_port, ctl: c_int) {
    static void tegra_tcu_uart_break_ctl(struct uart_port *port, int ctl)
    {
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_startup(port: *mut uart_port) -> c_int {
    static int tegra_tcu_uart_startup(struct uart_port *port)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_uart_shutdown(port: *mut uart_port) {
    static void tegra_tcu_uart_shutdown(struct uart_port *port)
    {
    }
    static void tegra_tcu_uart_set_termios(struct uart_port *port,
    struct ktermios *new,
    const struct ktermios *old)
    {
    }
    static const struct uart_ops tegra_tcu_uart_ops = {
    .tx_empty = tegra_tcu_uart_tx_empty,
    .set_mctrl = tegra_tcu_uart_set_mctrl,
    .get_mctrl = tegra_tcu_uart_get_mctrl,
    .stop_tx = tegra_tcu_uart_stop_tx,
    .start_tx = tegra_tcu_uart_start_tx,
    .stop_rx = tegra_tcu_uart_stop_rx,
    .break_ctl = tegra_tcu_uart_break_ctl,
    .startup = tegra_tcu_uart_startup,
    .shutdown = tegra_tcu_uart_shutdown,
    .set_termios = tegra_tcu_uart_set_termios,
    };

    static void tegra_tcu_console_write(struct console *cons, const char *s,
    unsigned int count)
    {
    struct tegra_tcu *tcu = container_of(cons, struct tegra_tcu, console);
    tegra_tcu_write(tcu, s, count);
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_console_setup(cons: *mut console, options: *mut c_char) -> c_int {
    static int tegra_tcu_console_setup(struct console *cons, char *options)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tegra_tcu_receive(cl: *mut mbox_client, msg: *mut c_void) {
    static void tegra_tcu_receive(struct mbox_client *cl, void *msg)
    {
    struct tegra_tcu *tcu = container_of(cl, struct tegra_tcu, rx_client);
    struct tty_port *port = &tcu.port.state.port;
    let mut value: u32 = (u32)(unsigned long)msg;
    unsigned int num_bytes, i;
    num_bytes = TCU_MBOX_NUM_BYTES_V(value);
    for (i = 0; i < num_bytes; i++)
    tty_insert_flip_char(port, TCU_MBOX_BYTE_V(value, i),
    TTY_NORMAL);
    tty_flip_buffer_push(port);
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_tcu_probe(struct platform_device *pdev)
    {
    struct uart_port *port;
    struct tegra_tcu *tcu;
    int err;
    tcu = devm_kzalloc(&pdev.dev, sizeof(*tcu), GFP_KERNEL);
    if (!tcu)
    return -ENOMEM;
    tcu.tx_client.dev = &pdev.dev;
    tcu.rx_client.dev = &pdev.dev;
    tcu.rx_client.rx_callback = tegra_tcu_receive;
    tcu.tx = mbox_request_channel_byname(&tcu.tx_client, "tx");
    if (IS_ERR(tcu.tx)) {
    err = PTR_ERR(tcu.tx);
    dev_err(&pdev.dev, "failed to get tx mailbox: %d\n", err);
    return err;
    }

// setup the console
    strcpy(tcu.console.name, "ttyTCU");
    tcu.console.device = uart_console_device;
    tcu.console.flags = CON_PRINTBUFFER | CON_ANYTIME;
    tcu.console.index = -1;
    tcu.console.write = tegra_tcu_console_write;
    tcu.console.setup = tegra_tcu_console_setup;
    tcu.console.data = &tcu.driver;

// setup the driver
    tcu.driver.owner = THIS_MODULE;
    tcu.driver.driver_name = "tegra-tcu";
    tcu.driver.dev_name = "ttyTCU";

    tcu.driver.cons = &tcu.console;

    tcu.driver.nr = 1;
    err = uart_register_driver(&tcu.driver);
    if (err) {
    dev_err(&pdev.dev, "failed to register UART driver: %d\n",
    err);
    goto free_tx;
    }
// setup the port
    port = &tcu.port;
    spin_lock_init(&port.lock);
    port.dev = &pdev.dev;
    port.type = PORT_TEGRA_TCU;
    port.ops = &tegra_tcu_uart_ops;
    port.fifosize = 1;
    port.iotype = UPIO_MEM;
    port.flags = UPF_BOOT_AUTOCONF;
    port.private_data = tcu;
    err = uart_add_one_port(&tcu.driver, port);
    if (err) {
    dev_err(&pdev.dev, "failed to add UART port: %d\n", err);
    goto unregister_uart;
    }
//
// Request RX channel after creating port to ensure tcu->port
// is ready for any immediate incoming bytes.
//
    tcu.rx = mbox_request_channel_byname(&tcu.rx_client, "rx");
    if (IS_ERR(tcu.rx)) {
    err = PTR_ERR(tcu.rx);
    dev_err(&pdev.dev, "failed to get rx mailbox: %d\n", err);
    goto remove_uart_port;
    }
    platform_set_drvdata(pdev, tcu);

    register_console(&tcu.console);

    return 0;
    remove_uart_port:
    uart_remove_one_port(&tcu.driver, &tcu.port);
    unregister_uart:
    uart_unregister_driver(&tcu.driver);
    free_tx:
    mbox_free_channel(tcu.tx);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra_tcu_remove(pdev: *mut platform_device) {
    static void tegra_tcu_remove(struct platform_device *pdev)
    {
    struct tegra_tcu *tcu = platform_get_drvdata(pdev);

    unregister_console(&tcu.console);

    mbox_free_channel(tcu.rx);
    uart_remove_one_port(&tcu.driver, &tcu.port);
    uart_unregister_driver(&tcu.driver);
    mbox_free_channel(tcu.tx);
    }
    static const struct of_device_id tegra_tcu_match[] = {
    { .compatible = "nvidia,tegra194-tcu" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tegra_tcu_match);
    static struct platform_driver tegra_tcu_driver = {
    .driver = {
    .name = "tegra-tcu",
    .of_match_table = tegra_tcu_match,
    },
    .probe = tegra_tcu_probe,
    .remove = tegra_tcu_remove,
    };
    module_platform_driver(tegra_tcu_driver);
    MODULE_AUTHOR("Mikko Perttunen <mperttunen@nvidia.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("NVIDIA Tegra Combined UART driver");
