//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-sibyte.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2004 Steven J. Hill
// Copyright (C) 2001,2002,2003 Broadcom Corporation
// Copyright (C) 1995-2000 Simon G. Vogl
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_algo_sibyte_data {
    pub /: *mut *mut *mut void data; / private data,
    pub /: *mut *mut int bus; / which bus,
    pub /: *mut *mut *mut void reg_base; / CSR base,
}

// ----- global defines -----------------------------------------------

    static int smbus_xfer(struct i2c_adapter *i2c_adap, u16 addr,
    unsigned short flags, char read_write,
    u8 command, int size, union i2c_smbus_data * data)
    {
    struct i2c_algo_sibyte_data *adap = i2c_adap.algo_data;
    let mut data_bytes: c_int = 0;
    int error;
    while (csr_in32(SMB_CSR(adap, R_SMB_STATUS)) & M_SMB_BUSY)
    ;
    switch (size) {
    case I2C_SMBUS_QUICK:
    csr_out32((V_SMB_ADDR(addr) |
    (read_write == I2C_SMBUS_READ ? M_SMB_QDATA : 0) |
    V_SMB_TT_QUICKCMD), SMB_CSR(adap, R_SMB_START));
    break;
    case I2C_SMBUS_BYTE:
    if (read_write == I2C_SMBUS_READ) {
    csr_out32((V_SMB_ADDR(addr) | V_SMB_TT_RD1BYTE),
    SMB_CSR(adap, R_SMB_START));
    data_bytes = 1;
    } else {
    csr_out32(V_SMB_CMD(command), SMB_CSR(adap, R_SMB_CMD));
    csr_out32((V_SMB_ADDR(addr) | V_SMB_TT_WR1BYTE),
    SMB_CSR(adap, R_SMB_START));
    }
    break;
    case I2C_SMBUS_BYTE_DATA:
    csr_out32(V_SMB_CMD(command), SMB_CSR(adap, R_SMB_CMD));
    if (read_write == I2C_SMBUS_READ) {
    csr_out32((V_SMB_ADDR(addr) | V_SMB_TT_CMD_RD1BYTE),
    SMB_CSR(adap, R_SMB_START));
    data_bytes = 1;
    } else {
    csr_out32(V_SMB_LB(data.byte),
    SMB_CSR(adap, R_SMB_DATA));
    csr_out32((V_SMB_ADDR(addr) | V_SMB_TT_WR2BYTE),
    SMB_CSR(adap, R_SMB_START));
    }
    break;
    case I2C_SMBUS_WORD_DATA:
    csr_out32(V_SMB_CMD(command), SMB_CSR(adap, R_SMB_CMD));
    if (read_write == I2C_SMBUS_READ) {
    csr_out32((V_SMB_ADDR(addr) | V_SMB_TT_CMD_RD2BYTE),
    SMB_CSR(adap, R_SMB_START));
    data_bytes = 2;
    } else {
    csr_out32(V_SMB_LB(data.word & 0xff),
    SMB_CSR(adap, R_SMB_DATA));
    csr_out32(V_SMB_MB(data.word >> 8),
    SMB_CSR(adap, R_SMB_DATA));
    csr_out32((V_SMB_ADDR(addr) | V_SMB_TT_WR2BYTE),
    SMB_CSR(adap, R_SMB_START));
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    while (csr_in32(SMB_CSR(adap, R_SMB_STATUS)) & M_SMB_BUSY)
    ;
    error = csr_in32(SMB_CSR(adap, R_SMB_STATUS));
    if (error & M_SMB_ERROR) {
// Clear error bit by writing a 1
    csr_out32(M_SMB_ERROR, SMB_CSR(adap, R_SMB_STATUS));
    return (error & M_SMB_ERROR_TYPE) ? -EIO : -ENXIO;
    }
    if (data_bytes == 1)
    data.byte = csr_in32(SMB_CSR(adap, R_SMB_DATA)) & 0xff;
    if (data_bytes == 2)
    data.word = csr_in32(SMB_CSR(adap, R_SMB_DATA)) & 0xffff;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bit_func(adap: *mut i2c_adapter) -> u32 {
    static u32 bit_func(struct i2c_adapter *adap)
    {
    return (I2C_FUNC_SMBUS_QUICK | I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA);
    }
// -----exported algorithm data: -------------------------------------
    static const struct i2c_algorithm i2c_sibyte_algo = {
    .smbus_xfer	= smbus_xfer,
    .functionality	= bit_func,
    };
//
// registering functions to load algorithms at runtime
//
#[no_mangle]
unsafe extern "C" fn i2c_sibyte_add_bus(i2c_adap: *mut i2c_adapter, speed: c_int) -> int __init {
    static int __init i2c_sibyte_add_bus(struct i2c_adapter *i2c_adap, int speed)
    {
    struct i2c_algo_sibyte_data *adap = i2c_adap.algo_data;
// Register new adapter to i2c module...
    i2c_adap.algo = &i2c_sibyte_algo;
// Set the requested frequency.
    csr_out32(speed, SMB_CSR(adap,R_SMB_FREQ));
    csr_out32(0, SMB_CSR(adap,R_SMB_CONTROL));
    return i2c_add_numbered_adapter(i2c_adap);
    }
    static struct i2c_algo_sibyte_data sibyte_board_data[2] = {
    { core::ptr::null_mut(), 0, (void *) (CKSEG1+A_SMB_BASE(0)) },
    { core::ptr::null_mut(), 1, (void *) (CKSEG1+A_SMB_BASE(1)) }
    };
    static struct i2c_adapter sibyte_board_adapter[2] = {
    {
    .owner		= THIS_MODULE,
    .class		= I2C_CLASS_HWMON,
    .algo		= core::ptr::null_mut(),
    .algo_data	= &sibyte_board_data[0],
    .nr		= 0,
    .name		= "SiByte SMBus 0",
    },
    {
    .owner		= THIS_MODULE,
    .class		= I2C_CLASS_HWMON,
    .algo		= core::ptr::null_mut(),
    .algo_data	= &sibyte_board_data[1],
    .nr		= 1,
    .name		= "SiByte SMBus 1",
    },
    };
#[no_mangle]
unsafe extern "C" fn i2c_sibyte_init() -> int __init {
    static int __init i2c_sibyte_init(void)
    {
    pr_info("i2c-sibyte: i2c SMBus adapter module for SiByte board\n");
    if (i2c_sibyte_add_bus(&sibyte_board_adapter[0], K_SMB_FREQ_100KHZ) < 0)
    return -ENODEV;
    if (i2c_sibyte_add_bus(&sibyte_board_adapter[1],
    K_SMB_FREQ_400KHZ) < 0) {
    i2c_del_adapter(&sibyte_board_adapter[0]);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_sibyte_exit() -> void __exit {
    static void __exit i2c_sibyte_exit(void)
    {
    i2c_del_adapter(&sibyte_board_adapter[0]);
    i2c_del_adapter(&sibyte_board_adapter[1]);
    }
    module_init(i2c_sibyte_init);
    module_exit(i2c_sibyte_exit);
    MODULE_AUTHOR("Kip Walker (Broadcom Corp.)");
    MODULE_AUTHOR("Steven J. Hill <sjhill@realitydiluted.com>");
    MODULE_DESCRIPTION("SMBus adapter routines for SiByte boards");
    MODULE_LICENSE("GPL");
