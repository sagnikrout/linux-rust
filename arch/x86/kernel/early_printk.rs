//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/early_printk.c
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

// Simple VGA output

    let mut max_ypos: static int = 25, max_xpos = 80;
    let mut current_ypos: static int = 25, current_xpos;
#[no_mangle]
unsafe extern "C" fn early_vga_write(con: *mut console, str: *const c_char, n: unsigned) {
    static void early_vga_write(struct console *con, const char *str, unsigned n)
    {
    char c;
    int  i, k, j;
    while ((c = *str++) != '\0' && n-- > 0) {
    if (current_ypos >= max_ypos) {
// scroll 1 line up
    for (k = 1, j = 0; k < max_ypos; k++, j++) {
    for (i = 0; i < max_xpos; i++) {
    writew(readw(VGABASE+2*(max_xpos*k+i)),
    VGABASE + 2*(max_xpos*j + i));
    }
    }
    for (i = 0; i < max_xpos; i++)
    writew(0x720, VGABASE + 2*(max_xpos*j + i));
    current_ypos = max_ypos-1;
    }

    if (c == '\b') {
    if (current_xpos > 0)
    current_xpos--;
    } else if (c == '\r') {
    current_xpos = 0;
    } else

    if (c == '\n') {
    current_xpos = 0;
    current_ypos++;
    } else if (c != '\r')  {
    writew(((0x7 << 8) | (unsigned short) c),
    VGABASE + 2*(max_xpos*current_ypos +
    current_xpos++));
    if (current_xpos >= max_xpos) {
    current_xpos = 0;
    current_ypos++;
    }
    }
    }
    }
    static struct console early_vga_console = {
    .name =		"earlyvga",
    .write =	early_vga_write,
    .flags =	CON_PRINTBUFFER,
    .index =	-1,
    };
// Serial functions loosely based on a similar package from Klaus P. Gerlicher
    static unsigned long early_serial_base = 0x3f8;  /* ttyS0 */
pub const XMTRDY: c_uint = 0x20;
pub const DLAB: c_uint = 0x80;

#[no_mangle]
unsafe extern "C" fn io_serial_in(addr: c_ulong, offset: c_int) -> __noendbr unsigned int {
    static __noendbr unsigned int io_serial_in(unsigned long addr, int offset)
    {
    return inb(addr + offset);
    }
    ANNOTATE_NOENDBR_SYM(io_serial_in);
#[no_mangle]
unsafe extern "C" fn io_serial_out(addr: c_ulong, offset: c_int, value: c_int) -> __noendbr void {
    static __noendbr void io_serial_out(unsigned long addr, int offset, int value)
    {
    outb(value, addr + offset);
    }
    ANNOTATE_NOENDBR_SYM(io_serial_out);
    DEFINE_STATIC_CALL(serial_in, io_serial_in);
    DEFINE_STATIC_CALL(serial_out, io_serial_out);
#[no_mangle]
unsafe extern "C" fn early_serial_putc(ch: c_uchar) -> c_int {
    static int early_serial_putc(unsigned char ch)
    {
    let mut timeout: unsigned = 0xffff;
    while ((static_call(serial_in)(early_serial_base, LSR) & XMTRDY) == 0 && --timeout)
    cpu_relax();
    static_call(serial_out)(early_serial_base, TXR, ch);
    return timeout ? 0 : -1;
    }
#[no_mangle]
unsafe extern "C" fn early_serial_write(con: *mut console, s: *const c_char, n: unsigned) {
    static void early_serial_write(struct console *con, const char *s, unsigned n)
    {
    while (*s && n-- > 0) {
    if (*s == '\n')
    early_serial_putc('\r');
    early_serial_putc(*s);
    s++;
    }
    }
#[no_mangle]
unsafe extern "C" fn early_serial_hw_init(divisor: unsigned) -> __init void {
    static __init void early_serial_hw_init(unsigned divisor)
    {
    unsigned char c;
    static_call(serial_out)(early_serial_base, LCR, 0x3);	/* 8n1 */
    static_call(serial_out)(early_serial_base, IER, 0);	/* no interrupt */
    static_call(serial_out)(early_serial_base, FCR, 0);	/* no fifo */
    static_call(serial_out)(early_serial_base, MCR, 0x3);	/* DTR + RTS */
    c = static_call(serial_in)(early_serial_base, LCR);
    static_call(serial_out)(early_serial_base, LCR, c | DLAB);
    static_call(serial_out)(early_serial_base, DLL, divisor & 0xff);
    static_call(serial_out)(early_serial_base, DLH, (divisor >> 8) & 0xff);
    static_call(serial_out)(early_serial_base, LCR, c & ~DLAB);

    if (static_call_query(serial_in) == io_serial_in)
    kexec_debug_8250_port = early_serial_base;

    }
pub const DEFAULT_BAUD: c_int = 9600;
#[no_mangle]
unsafe extern "C" fn early_serial_init(s: *mut c_char) -> __init void {
    static __init void early_serial_init(char *s)
    {
    unsigned divisor;
    let mut baud: c_ulong = DEFAULT_BAUD;
    char *e;
    if (*s == ',')
    ++s;
    if (*s) {
    unsigned port;
    if (!strncmp(s, "0x", 2)) {
    early_serial_base = simple_strtoul(s, &e, 16);
    } else {
    static const int __initconst bases[] = { 0x3f8, 0x2f8 };
    if (!strncmp(s, "ttyS", 4))
    s += 4;
    port = simple_strtoul(s, &e, 10);
    if (port > 1 || s == e)
    port = 0;
    early_serial_base = bases[port];
    }
    s += strcspn(s, ",");
    if (*s == ',')
    s++;
    }
    if (*s) {
    baud = simple_strtoull(s, &e, 0);
    if (baud == 0 || s == e)
    baud = DEFAULT_BAUD;
    }
// Convert from baud to divisor value
    divisor = 115200 / baud;
// Set up the HW
    early_serial_hw_init(divisor);
    }
#[no_mangle]
unsafe extern "C" fn mem32_serial_out(addr: c_ulong, offset: c_int, value: c_int) -> __noendbr void {
    static __noendbr void mem32_serial_out(unsigned long addr, int offset, int value)
    {
    u32 __iomem *vaddr = (u32 __iomem *)addr;
// shift implied by pointer type
    writel(value, vaddr + offset);
    }
    ANNOTATE_NOENDBR_SYM(mem32_serial_out);
#[no_mangle]
unsafe extern "C" fn mem32_serial_in(addr: c_ulong, offset: c_int) -> __noendbr unsigned int {
    static __noendbr unsigned int mem32_serial_in(unsigned long addr, int offset)
    {
    u32 __iomem *vaddr = (u32 __iomem *)addr;
// shift implied by pointer type
    return readl(vaddr + offset);
    }
    ANNOTATE_NOENDBR_SYM(mem32_serial_in);
//
// early_mmio_serial_init() - Initialize MMIO-based early serial console.
// @s: MMIO-based serial specification.
//
#[no_mangle]
unsafe extern "C" fn early_mmio_serial_init(s: *mut c_char) -> __init void {
    static __init void early_mmio_serial_init(char *s)
    {
    unsigned long baudrate;
    unsigned long membase;
    char *e;
    if (*s == ',')
    s++;
    if (!strncmp(s, "0x", 2)) {
// NB: only 32-bit addresses are supported.
    membase = simple_strtoul(s, &e, 16);
    early_serial_base = (unsigned long)early_ioremap(membase, PAGE_SIZE);
    static_call_update(serial_in, mem32_serial_in);
    static_call_update(serial_out, mem32_serial_out);
    s += strcspn(s, ",");
    if (*s == ',')
    s++;
    }
    if (!strncmp(s, "nocfg", 5)) {
    baudrate = 0;
    } else {
    baudrate = simple_strtoul(s, &e, 0);
    if (baudrate == 0 || s == e)
    baudrate = DEFAULT_BAUD;
    }
    if (baudrate)
    early_serial_hw_init(115200 / baudrate);
    }

//
// early_pci_serial_init()
//
// This function is invoked when the early_printk param starts with "pciserial"
// The rest of the param should be "[force],B:D.F,baud", where B, D & F describe
// the location of a PCI device that must be a UART device. "force" is optional
// and overrides the use of an UART device with a wrong PCI class code.
//
#[no_mangle]
unsafe extern "C" fn early_pci_serial_init(s: *mut c_char) -> __init void {
    static __init void early_pci_serial_init(char *s)
    {
    unsigned divisor;
    let mut baud: c_ulong = DEFAULT_BAUD;
    u8 bus, slot, func;
    u32 classcode, bar0;
    u16 cmdreg;
    char *e;
    let mut force: c_int = 0;
    if (*s == ',')
    ++s;
    if (*s == 0)
    return;
// Force the use of an UART device with wrong class code
    if (!strncmp(s, "force,", 6)) {
    force = 1;
    s += 6;
    }
//
// Part the param to get the BDF values
//
    bus = (u8)simple_strtoul(s, &e, 16);
    s = e;
    if (*s != ':')
    return;
    ++s;
    slot = (u8)simple_strtoul(s, &e, 16);
    s = e;
    if (*s != '.')
    return;
    ++s;
    func = (u8)simple_strtoul(s, &e, 16);
    s = e;
// A baud might be following
    if (*s == ',')
    s++;
//
// Find the device from the BDF
//
    cmdreg = read_pci_config(bus, slot, func, PCI_COMMAND);
    classcode = read_pci_config(bus, slot, func, PCI_CLASS_REVISION);
    bar0 = read_pci_config(bus, slot, func, PCI_BASE_ADDRESS_0);
//
// Verify it is a 16550-UART type device
//
    if (((classcode >> 16 != PCI_CLASS_COMMUNICATION_MODEM) &&
    (classcode >> 16 != PCI_CLASS_COMMUNICATION_SERIAL)) ||
    (((classcode >> 8) & 0xff) != PCI_SERIAL_16550_COMPATIBLE)) {
    if (!force)
    return;
    }
//
// Determine if it is IO or memory mapped
//
    if ((bar0 & PCI_BASE_ADDRESS_SPACE) == PCI_BASE_ADDRESS_SPACE_IO) {
// it is IO mapped
    early_serial_base = bar0 & PCI_BASE_ADDRESS_IO_MASK;
    write_pci_config(bus, slot, func, PCI_COMMAND,
    cmdreg|PCI_COMMAND_IO);
    } else {
// It is memory mapped - assume 32-bit alignment
    static_call_update(serial_in, mem32_serial_in);
    static_call_update(serial_out, mem32_serial_out);
// WARNING! assuming the address is always in the first 4G
    early_serial_base =
    (unsigned long)early_ioremap(bar0 & PCI_BASE_ADDRESS_MEM_MASK, 0x10);

    kexec_debug_8250_mmio32 = bar0 & PCI_BASE_ADDRESS_MEM_MASK;

    write_pci_config(bus, slot, func, PCI_COMMAND,
    cmdreg|PCI_COMMAND_MEMORY);
    }
//
// Initialize the hardware
//
    if (*s) {
    if (strcmp(s, "nocfg") == 0)
// Sometimes, we want to leave the UART alone
// and assume the BIOS has set it up correctly.
// "nocfg" tells us this is the case, and we
// should do no more setup.
//
    return;
    if (kstrtoul(s, 0, &baud) < 0 || baud == 0)
    baud = DEFAULT_BAUD;
    }
// Convert from baud to divisor value
    divisor = 115200 / baud;
// Set up the HW
    early_serial_hw_init(divisor);
    }

    static struct console early_serial_console = {
    .name =		"earlyser",
    .write =	early_serial_write,
    .flags =	CON_PRINTBUFFER,
    .index =	-1,
    };
#[no_mangle]
unsafe extern "C" fn early_console_register(con: *mut console, keep_early: c_int) {
    static void early_console_register(struct console *con, int keep_early)
    {
    if (con.index != -1) {
    printk(KERN_CRIT "ERROR: earlyprintk= %s already used\n",
    con.name);
    return;
    }
    early_console = con;
    if (keep_early)
    early_console.flags &= ~CON_BOOT;
    else
    early_console.flags |= CON_BOOT;
    register_console(early_console);
    }
#[no_mangle]
unsafe extern "C" fn setup_early_printk(buf: *mut c_char) -> int __init {
    static int __init setup_early_printk(char *buf)
    {
    int keep;
    if (!buf)
    return 0;
    if (early_console)
    return 0;
    keep = (strstr(buf, "keep") != core::ptr::null_mut());
    while (*buf != '\0') {
    if (!strncmp(buf, "mmio32", 6)) {
    buf += 6;
    early_mmio_serial_init(buf);
    early_console_register(&early_serial_console, keep);
    }
    if (!strncmp(buf, "serial", 6)) {
    buf += 6;
    early_serial_init(buf);
    early_console_register(&early_serial_console, keep);
    if (!strncmp(buf, ",ttyS", 5))
    buf += 5;
    }
    if (!strncmp(buf, "ttyS", 4)) {
    early_serial_init(buf + 4);
    early_console_register(&early_serial_console, keep);
    }

    if (!strncmp(buf, "pciserial", 9)) {
    buf += 9; /* Keep from match the above "pciserial" */
    early_pci_serial_init(buf);
    early_console_register(&early_serial_console, keep);
    }

    if (!strncmp(buf, "vga", 3) &&
    boot_params.screen_info.orig_video_isVGA == 1) {
    max_xpos = boot_params.screen_info.orig_video_cols;
    max_ypos = boot_params.screen_info.orig_video_lines;
    current_ypos = boot_params.screen_info.orig_y;
    early_console_register(&early_vga_console, keep);
    }

    if (!strncmp(buf, "dbgp", 4) && !early_dbgp_init(buf + 4))
    early_console_register(&early_dbgp_console, keep);

    if (!strncmp(buf, "xen", 3))
    early_console_register(&xenboot_console, keep);

    if (!strncmp(buf, "xdbc", 4))
    early_xdbc_parse_parameter(buf + 4, keep);

    buf++;
    }
    return 0;
    }
    early_param("earlyprintk", setup_early_printk);
