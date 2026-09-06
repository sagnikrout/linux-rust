//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/serial.c
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
// Generic serial console support
//
// Author: Mark A. Greer <mgreer@mvista.com>
//
// Code in serial_edit_cmdline() copied from <file:arch/ppc/boot/simple/misc.c>
// and was written by Matt Porter <mporter@kernel.crashing.org>.
//
// 2001,2006 (c) MontaVista Software, Inc.
//

#[no_mangle]
unsafe extern "C" fn serial_open() -> c_int {
    static int serial_open(void)
    {
    struct serial_console_data *scdp = console_ops.data;
    return scdp.open();
    }
#[no_mangle]
unsafe extern "C" fn serial_write(buf: *const c_char, len: c_int) {
    static void serial_write(const char *buf, int len)
    {
    struct serial_console_data *scdp = console_ops.data;
    while (*buf != '\0')
    scdp.putc(*buf++);
    }
#[no_mangle]
unsafe extern "C" fn serial_edit_cmdline(buf: *mut c_char, len: c_int, timeout: c_uint) {
    static void serial_edit_cmdline(char *buf, int len, unsigned int timeout)
    {
    let mut timer: c_int = 0, count;
    char ch, *cp;
    struct serial_console_data *scdp = console_ops.data;
    count = strlen(buf);
    cp = &buf[count];
    count++;
    do {
    if (scdp.tstc()) {
    while (((ch = scdp.getc()) != '\n') && (ch != '\r')) {
// Test for backspace/delete
    if ((ch == '\b') || (ch == '\177')) {
    if (cp != buf) {
    cp--;
    count--;
    printf("\b \b");
    }
// Test for ^x/^u (and wipe the line)
    } else if ((ch == '\030') || (ch == '\025')) {
    while (cp != buf) {
    cp--;
    count--;
    printf("\b \b");
    }
    } else if (count < len) {
// cp++ = ch;
    count++;
    scdp.putc(ch);
    }
    }
    break;  /* Exit 'timer' loop */
    }
    udelay(1000);  /* 1 msec */
    } while (timer++ < timeout);
// cp = 0;
    }
#[no_mangle]
unsafe extern "C" fn serial_close() {
    static void serial_close(void)
    {
    struct serial_console_data *scdp = console_ops.data;
    if (scdp.close)
    scdp.close();
    }
    static void *serial_get_stdout_devp(void)
    {
    void *devp;
    char devtype[MAX_PROP_LEN];
    char path[MAX_PATH_LEN];
    devp = finddevice("/chosen");
    if (devp == core::ptr::null_mut())
    goto err_out;
    if (getprop(devp, "linux,stdout-path", path, MAX_PATH_LEN) > 0 ||
    getprop(devp, "stdout-path", path, MAX_PATH_LEN) > 0) {
    devp = finddevice(path);
    if (devp == core::ptr::null_mut())
    goto err_out;
    if ((getprop(devp, "device_type", devtype, sizeof(devtype)) > 0)
    && !strcmp(devtype, "serial"))
    return devp;
    }
    err_out:
    return core::ptr::null_mut();
    }
    static struct serial_console_data serial_cd;
// Node's "compatible" property determines which serial driver to use
#[no_mangle]
pub unsafe extern "C" fn serial_console_init() -> c_int {
    int serial_console_init(void)
    {
    void *devp;
    let mut rc: c_int = -1;
    devp = serial_get_stdout_devp();
    if (devp == core::ptr::null_mut())
    goto err_out;
    if (dt_is_compatible(devp, "ns16550") ||
    dt_is_compatible(devp, "pnpPNP,501"))
    rc = ns16550_console_init(devp, &serial_cd);

    else if (dt_is_compatible(devp, "fsl,cpm1-scc-uart") ||
    dt_is_compatible(devp, "fsl,cpm1-smc-uart") ||
    dt_is_compatible(devp, "fsl,cpm2-scc-uart") ||
    dt_is_compatible(devp, "fsl,cpm2-smc-uart"))
    rc = cpm_console_init(devp, &serial_cd);

#[no_mangle]
pub unsafe extern "C" fn if(_arg: dt_is_compatible(devp, _arg: "fsl, _arg: mpc5200-psc-uart")) -> else {
    else if (dt_is_compatible(devp, "fsl,mpc5200-psc-uart"))
    rc = mpc5200_psc_console_init(devp, &serial_cd);

#[no_mangle]
pub unsafe extern "C" fn if(_arg: dt_is_compatible(devp, _arg: "ibm, _arg: opal-console-raw")) -> else {
    else if (dt_is_compatible(devp, "ibm,opal-console-raw"))
    rc = opal_console_init(devp, &serial_cd);

// Add other serial console driver calls here
    if (!rc) {
    console_ops.open = serial_open;
    console_ops.write = serial_write;
    console_ops.close = serial_close;
    console_ops.data = &serial_cd;
    if (serial_cd.getc)
    console_ops.edit_cmdline = serial_edit_cmdline;
    return 0;
    }
    err_out:
    return -1;
    }
