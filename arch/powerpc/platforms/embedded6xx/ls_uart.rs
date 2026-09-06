//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/embedded6xx/ls_uart.c
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


//
// AVR power-management chip interface for the Buffalo Linkstation
// Kurobox Platform.
//
// Author: 2006 (c) G. Liakhovetski
// g.liakhovetski@gmx.de
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of
// any kind, whether express or implied.
//

    static void __iomem *avr_addr;
    static unsigned long avr_clock;
    static struct work_struct wd_work;
#[no_mangle]
unsafe extern "C" fn wd_stop(unused: *mut work_struct) {
    static void wd_stop(struct work_struct *unused)
    {
    const char string[] = "AAAAFFFFJJJJ>>>>VVVV>>>>ZZZZVVVVKKKK";
    let mut i: c_int = 0, rescue = 8;
    let mut len: c_int = strlen(string);
    while (rescue--) {
    int j;
    let mut lsr: c_char = in_8(avr_addr + UART_LSR);
    if (lsr & (UART_LSR_THRE | UART_LSR_TEMT)) {
    for (j = 0; j < 16 && i < len; j++, i++)
    out_8(avr_addr + UART_TX, string[i]);
    if (i == len) {
// Read "OK" back: 4ms for the last "KKKK"
    plus a couple bytes back */
    msleep(7);
    printk("linkstation: disarming the AVR watchdog: ");
    while (in_8(avr_addr + UART_LSR) & UART_LSR_DR)
    printk("%c", in_8(avr_addr + UART_RX));
    break;
    }
    }
    msleep(17);
    }
    printk("\n");
    }

#[no_mangle]
pub unsafe extern "C" fn avr_uart_configure() {
    void avr_uart_configure(void)
    {
    let mut cval: c_uchar = UART_LCR_WLEN8;
    let mut quot: c_uint = AVR_QUOT(avr_clock);
    if (!avr_addr || !avr_clock)
    return;
    out_8(avr_addr + UART_LCR, cval);			/* initialise UART */
    out_8(avr_addr + UART_MCR, 0);
    out_8(avr_addr + UART_IER, 0);
    cval |= UART_LCR_STOP | UART_LCR_PARITY | UART_LCR_EPAR;
    out_8(avr_addr + UART_LCR, cval);			/* Set character format */
    out_8(avr_addr + UART_LCR, cval | UART_LCR_DLAB);	/* set DLAB */
    out_8(avr_addr + UART_DLL, quot & 0xff);		/* LS of divisor */
    out_8(avr_addr + UART_DLM, quot >> 8);			/* MS of divisor */
    out_8(avr_addr + UART_LCR, cval);			/* reset DLAB */
    out_8(avr_addr + UART_FCR, UART_FCR_ENABLE_FIFO);	/* enable FIFO */
    }
#[no_mangle]
pub unsafe extern "C" fn avr_uart_send(c: c_char) {
    void avr_uart_send(const char c)
    {
    if (!avr_addr || !avr_clock)
    return;
    out_8(avr_addr + UART_TX, c);
    out_8(avr_addr + UART_TX, c);
    out_8(avr_addr + UART_TX, c);
    out_8(avr_addr + UART_TX, c);
    }
#[no_mangle]
unsafe extern "C" fn ls_uart_init() -> void __init {
    static void __init ls_uart_init(void)
    {
    local_irq_disable();

    out_8(avr_addr + UART_FCR, UART_FCR_ENABLE_FIFO);	/* enable FIFO */
    out_8(avr_addr + UART_FCR, UART_FCR_ENABLE_FIFO |
    UART_FCR_CLEAR_RCVR | UART_FCR_CLEAR_XMIT);	/* clear FIFOs */
    out_8(avr_addr + UART_FCR, 0);
    out_8(avr_addr + UART_IER, 0);
// Clear up interrupts
    (void) in_8(avr_addr + UART_LSR);
    (void) in_8(avr_addr + UART_RX);
    (void) in_8(avr_addr + UART_IIR);
    (void) in_8(avr_addr + UART_MSR);

    avr_uart_configure();
    local_irq_enable();
    }
#[no_mangle]
unsafe extern "C" fn ls_uarts_init() -> int __init {
    static int __init ls_uarts_init(void)
    {
    struct device_node *avr;
    struct resource res;
    int len, ret;
    avr = of_find_node_by_path("/soc10x/serial@80004500");
    if (!avr)
    return -EINVAL;
    avr_clock = *(u32*)of_get_property(avr, "clock-frequency", &len);
    if (!avr_clock)
    return -EINVAL;
    ret = of_address_to_resource(avr, 0, &res);
    if (ret)
    return ret;
    of_node_put(avr);
    avr_addr = ioremap(res.start, 32);
    if (!avr_addr)
    return -EFAULT;
    ls_uart_init();
    INIT_WORK(&wd_work, wd_stop);
    schedule_work(&wd_work);
    return 0;
    }
    machine_late_initcall(linkstation, ls_uarts_init);
