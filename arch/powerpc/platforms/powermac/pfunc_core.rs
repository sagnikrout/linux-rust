//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powermac/pfunc_core.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// FIXME: Properly make this race free with refcounting etc...
//
// FIXME: LOCKING !!!
//

// Debug
// Macro flag: #define LOG_PARSE(fmt...)

// Macro flag: #define LOG_BLOB(t,b,c)

// Macro flag: #define DBG(fmt...)

// Command numbers
pub const PMF_CMD_LIST: c_int = 0;
pub const PMF_CMD_WRITE_GPIO: c_int = 1;
pub const PMF_CMD_READ_GPIO: c_int = 2;
pub const PMF_CMD_WRITE_REG32: c_int = 3;
pub const PMF_CMD_READ_REG32: c_int = 4;
pub const PMF_CMD_WRITE_REG16: c_int = 5;
pub const PMF_CMD_READ_REG16: c_int = 6;
pub const PMF_CMD_WRITE_REG8: c_int = 7;
pub const PMF_CMD_READ_REG8: c_int = 8;
pub const PMF_CMD_DELAY: c_int = 9;
pub const PMF_CMD_WAIT_REG32: c_int = 10;
pub const PMF_CMD_WAIT_REG16: c_int = 11;
pub const PMF_CMD_WAIT_REG8: c_int = 12;
pub const PMF_CMD_READ_I2C: c_int = 13;
pub const PMF_CMD_WRITE_I2C: c_int = 14;
pub const PMF_CMD_RMW_I2C: c_int = 15;
pub const PMF_CMD_GEN_I2C: c_int = 16;
pub const PMF_CMD_SHIFT_BYTES_RIGHT: c_int = 17;
pub const PMF_CMD_SHIFT_BYTES_LEFT: c_int = 18;
pub const PMF_CMD_READ_CFG: c_int = 19;
pub const PMF_CMD_WRITE_CFG: c_int = 20;
pub const PMF_CMD_RMW_CFG: c_int = 21;
pub const PMF_CMD_READ_I2C_SUBADDR: c_int = 22;
pub const PMF_CMD_WRITE_I2C_SUBADDR: c_int = 23;
pub const PMF_CMD_SET_I2C_MODE: c_int = 24;
pub const PMF_CMD_RMW_I2C_SUBADDR: c_int = 25;
pub const PMF_CMD_READ_REG32_MASK_SHR_XOR: c_int = 26;
pub const PMF_CMD_READ_REG16_MASK_SHR_XOR: c_int = 27;
pub const PMF_CMD_READ_REG8_MASK_SHR_XOR: c_int = 28;
pub const PMF_CMD_WRITE_REG32_SHL_MASK: c_int = 29;
pub const PMF_CMD_WRITE_REG16_SHL_MASK: c_int = 30;
pub const PMF_CMD_WRITE_REG8_SHL_MASK: c_int = 31;
pub const PMF_CMD_MASK_AND_COMPARE: c_int = 32;
pub const PMF_CMD_COUNT: c_int = 33;
// This structure holds the state of the parser while walking through
// a function definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_cmd {
    pub cmdptr: *const c_void,
    pub cmdend: *const c_void,
    pub func: *mut pmf_function,
    pub instdata: *mut c_void,
    pub args: *mut pmf_args,
    pub error: c_int,
}

// Debug output
#[no_mangle]
unsafe extern "C" fn print_blob(title: *const c_char, blob: *const c_void, bytes: c_int) {
    static void print_blob(const char *title, const void *blob, int bytes)
    {
    printk("%s", title);
    while(bytes--) {
    printk("%02x ", *((u8 *)blob));
    blob += 1;
    }
    printk("\n");
    }

//
// Parser helpers
//
#[no_mangle]
unsafe extern "C" fn pmf_next32(cmd: *mut pmf_cmd) -> u32 {
    static u32 pmf_next32(struct pmf_cmd *cmd)
    {
    u32 value;
    if ((cmd.cmdend - cmd.cmdptr) < 4) {
    cmd.error = 1;
    return 0;
    }
    value = *((u32 *)cmd.cmdptr);
    cmd.cmdptr += 4;
    return value;
    }
#[no_mangle]
unsafe extern "C" fn pmf_next_blob(cmd: *mut pmf_cmd, count: c_int) -> *const c_void {
    static const void* pmf_next_blob(struct pmf_cmd *cmd, int count)
    {
    const void *value;
    if ((cmd.cmdend - cmd.cmdptr) < count) {
    cmd.error = 1;
    return core::ptr::null_mut();
    }
    value = cmd.cmdptr;
    cmd.cmdptr += count;
    return value;
    }
//
// Individual command parsers
//

    do { \
    if (cmd.error) \
    return -ENXIO; \
    if (handlers == core::ptr::null_mut()) \
    return 0; \
    if (handlers.name)				      \
    return handlers.name(cmd.func, cmd.instdata, \
    cmd.args, p);	      \
    return -1; \
    } while(0) \
#[no_mangle]
unsafe extern "C" fn pmf_parser_write_gpio(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_write_gpio(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut value: u8 = (u8)pmf_next32(cmd);
    let mut mask: u8 = (u8)pmf_next32(cmd);
    LOG_PARSE("pmf: write_gpio(value: %02x, mask: %02x)\n", value, mask);
    PMF_PARSE_CALL(write_gpio, cmd, h, value, mask);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_read_gpio(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_read_gpio(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut mask: u8 = (u8)pmf_next32(cmd);
    let mut rshift: c_int = (int)pmf_next32(cmd);
    let mut xor: u8 = (u8)pmf_next32(cmd);
    LOG_PARSE("pmf: read_gpio(mask: %02x, rshift: %d, xor: %02x)\n",
    mask, rshift, xor);
    PMF_PARSE_CALL(read_gpio, cmd, h, mask, rshift, xor);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_write_reg32(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_write_reg32(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut value: u32 = pmf_next32(cmd);
    let mut mask: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: write_reg32(offset: %08x, value: %08x, mask: %08x)\n",
    offset, value, mask);
    PMF_PARSE_CALL(write_reg32, cmd, h, offset, value, mask);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_read_reg32(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_read_reg32(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_reg32(offset: %08x)\n", offset);
    PMF_PARSE_CALL(read_reg32, cmd, h, offset);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_write_reg16(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_write_reg16(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut value: u16 = (u16)pmf_next32(cmd);
    let mut mask: u16 = (u16)pmf_next32(cmd);
    LOG_PARSE("pmf: write_reg16(offset: %08x, value: %04x, mask: %04x)\n",
    offset, value, mask);
    PMF_PARSE_CALL(write_reg16, cmd, h, offset, value, mask);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_read_reg16(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_read_reg16(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_reg16(offset: %08x)\n", offset);
    PMF_PARSE_CALL(read_reg16, cmd, h, offset);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_write_reg8(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_write_reg8(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut value: u8 = (u16)pmf_next32(cmd);
    let mut mask: u8 = (u16)pmf_next32(cmd);
    LOG_PARSE("pmf: write_reg8(offset: %08x, value: %02x, mask: %02x)\n",
    offset, value, mask);
    PMF_PARSE_CALL(write_reg8, cmd, h, offset, value, mask);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_read_reg8(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_read_reg8(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_reg8(offset: %08x)\n", offset);
    PMF_PARSE_CALL(read_reg8, cmd, h, offset);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_delay(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_delay(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut duration: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: delay(duration: %d us)\n", duration);
    PMF_PARSE_CALL(delay, cmd, h, duration);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_wait_reg32(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_wait_reg32(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut value: u32 = pmf_next32(cmd);
    let mut mask: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: wait_reg32(offset: %08x, comp_value: %08x,mask: %08x)\n",
    offset, value, mask);
    PMF_PARSE_CALL(wait_reg32, cmd, h, offset, value, mask);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_wait_reg16(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_wait_reg16(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut value: u16 = (u16)pmf_next32(cmd);
    let mut mask: u16 = (u16)pmf_next32(cmd);
    LOG_PARSE("pmf: wait_reg16(offset: %08x, comp_value: %04x,mask: %04x)\n",
    offset, value, mask);
    PMF_PARSE_CALL(wait_reg16, cmd, h, offset, value, mask);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_wait_reg8(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_wait_reg8(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut value: u8 = (u8)pmf_next32(cmd);
    let mut mask: u8 = (u8)pmf_next32(cmd);
    LOG_PARSE("pmf: wait_reg8(offset: %08x, comp_value: %02x,mask: %02x)\n",
    offset, value, mask);
    PMF_PARSE_CALL(wait_reg8, cmd, h, offset, value, mask);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_read_i2c(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_read_i2c(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut bytes: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_i2c(bytes: %ud)\n", bytes);
    PMF_PARSE_CALL(read_i2c, cmd, h, bytes);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_write_i2c(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_write_i2c(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut bytes: u32 = pmf_next32(cmd);
    const void *blob = pmf_next_blob(cmd, bytes);
    LOG_PARSE("pmf: write_i2c(bytes: %ud) ...\n", bytes);
    LOG_BLOB("pmf:   data: \n", blob, bytes);
    PMF_PARSE_CALL(write_i2c, cmd, h, bytes, blob);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_rmw_i2c(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_rmw_i2c(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut maskbytes: u32 = pmf_next32(cmd);
    let mut valuesbytes: u32 = pmf_next32(cmd);
    let mut totalbytes: u32 = pmf_next32(cmd);
    const void *maskblob = pmf_next_blob(cmd, maskbytes);
    const void *valuesblob = pmf_next_blob(cmd, valuesbytes);
    LOG_PARSE("pmf: rmw_i2c(maskbytes: %ud, valuebytes: %ud, "
    "totalbytes: %d) ...\n",
    maskbytes, valuesbytes, totalbytes);
    LOG_BLOB("pmf:   mask data: \n", maskblob, maskbytes);
    LOG_BLOB("pmf:   values data: \n", valuesblob, valuesbytes);
    PMF_PARSE_CALL(rmw_i2c, cmd, h, maskbytes, valuesbytes, totalbytes,
    maskblob, valuesblob);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_read_cfg(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_read_cfg(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut bytes: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_cfg(offset: %x, bytes: %ud)\n", offset, bytes);
    PMF_PARSE_CALL(read_cfg, cmd, h, offset, bytes);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_write_cfg(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_write_cfg(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut bytes: u32 = pmf_next32(cmd);
    const void *blob = pmf_next_blob(cmd, bytes);
    LOG_PARSE("pmf: write_cfg(offset: %x, bytes: %ud)\n", offset, bytes);
    LOG_BLOB("pmf:   data: \n", blob, bytes);
    PMF_PARSE_CALL(write_cfg, cmd, h, offset, bytes, blob);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_rmw_cfg(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_rmw_cfg(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut maskbytes: u32 = pmf_next32(cmd);
    let mut valuesbytes: u32 = pmf_next32(cmd);
    let mut totalbytes: u32 = pmf_next32(cmd);
    const void *maskblob = pmf_next_blob(cmd, maskbytes);
    const void *valuesblob = pmf_next_blob(cmd, valuesbytes);
    LOG_PARSE("pmf: rmw_cfg(maskbytes: %ud, valuebytes: %ud,"
    " totalbytes: %d) ...\n",
    maskbytes, valuesbytes, totalbytes);
    LOG_BLOB("pmf:   mask data: \n", maskblob, maskbytes);
    LOG_BLOB("pmf:   values data: \n", valuesblob, valuesbytes);
    PMF_PARSE_CALL(rmw_cfg, cmd, h, offset, maskbytes, valuesbytes,
    totalbytes, maskblob, valuesblob);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_read_i2c_sub(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_read_i2c_sub(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut subaddr: u8 = (u8)pmf_next32(cmd);
    let mut bytes: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_i2c_sub(subaddr: %x, bytes: %ud)\n",
    subaddr, bytes);
    PMF_PARSE_CALL(read_i2c_sub, cmd, h, subaddr, bytes);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_write_i2c_sub(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_write_i2c_sub(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut subaddr: u8 = (u8)pmf_next32(cmd);
    let mut bytes: u32 = pmf_next32(cmd);
    const void *blob = pmf_next_blob(cmd, bytes);
    LOG_PARSE("pmf: write_i2c_sub(subaddr: %x, bytes: %ud) ...\n",
    subaddr, bytes);
    LOG_BLOB("pmf:   data: \n", blob, bytes);
    PMF_PARSE_CALL(write_i2c_sub, cmd, h, subaddr, bytes, blob);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_set_i2c_mode(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_set_i2c_mode(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut mode: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: set_i2c_mode(mode: %d)\n", mode);
    PMF_PARSE_CALL(set_i2c_mode, cmd, h, mode);
    }
#[no_mangle]
unsafe extern "C" fn pmf_parser_rmw_i2c_sub(cmd: *mut pmf_cmd, h: *mut pmf_handlers) -> c_int {
    static int pmf_parser_rmw_i2c_sub(struct pmf_cmd *cmd, struct pmf_handlers *h)
    {
    let mut subaddr: u8 = (u8)pmf_next32(cmd);
    let mut maskbytes: u32 = pmf_next32(cmd);
    let mut valuesbytes: u32 = pmf_next32(cmd);
    let mut totalbytes: u32 = pmf_next32(cmd);
    const void *maskblob = pmf_next_blob(cmd, maskbytes);
    const void *valuesblob = pmf_next_blob(cmd, valuesbytes);
    LOG_PARSE("pmf: rmw_i2c_sub(subaddr: %x, maskbytes: %ud, valuebytes: %ud"
    ", totalbytes: %d) ...\n",
    subaddr, maskbytes, valuesbytes, totalbytes);
    LOG_BLOB("pmf:   mask data: \n", maskblob, maskbytes);
    LOG_BLOB("pmf:   values data: \n", valuesblob, valuesbytes);
    PMF_PARSE_CALL(rmw_i2c_sub, cmd, h, subaddr, maskbytes, valuesbytes,
    totalbytes, maskblob, valuesblob);
    }
    static int pmf_parser_read_reg32_msrx(struct pmf_cmd *cmd,
    struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut mask: u32 = pmf_next32(cmd);
    let mut shift: u32 = pmf_next32(cmd);
    let mut xor: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_reg32_msrx(offset: %x, mask: %x, shift: %x,"
    " xor: %x\n", offset, mask, shift, xor);
    PMF_PARSE_CALL(read_reg32_msrx, cmd, h, offset, mask, shift, xor);
    }
    static int pmf_parser_read_reg16_msrx(struct pmf_cmd *cmd,
    struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut mask: u32 = pmf_next32(cmd);
    let mut shift: u32 = pmf_next32(cmd);
    let mut xor: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_reg16_msrx(offset: %x, mask: %x, shift: %x,"
    " xor: %x\n", offset, mask, shift, xor);
    PMF_PARSE_CALL(read_reg16_msrx, cmd, h, offset, mask, shift, xor);
    }
    static int pmf_parser_read_reg8_msrx(struct pmf_cmd *cmd,
    struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut mask: u32 = pmf_next32(cmd);
    let mut shift: u32 = pmf_next32(cmd);
    let mut xor: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: read_reg8_msrx(offset: %x, mask: %x, shift: %x,"
    " xor: %x\n", offset, mask, shift, xor);
    PMF_PARSE_CALL(read_reg8_msrx, cmd, h, offset, mask, shift, xor);
    }
    static int pmf_parser_write_reg32_slm(struct pmf_cmd *cmd,
    struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut shift: u32 = pmf_next32(cmd);
    let mut mask: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: write_reg32_slm(offset: %x, shift: %x, mask: %x\n",
    offset, shift, mask);
    PMF_PARSE_CALL(write_reg32_slm, cmd, h, offset, shift, mask);
    }
    static int pmf_parser_write_reg16_slm(struct pmf_cmd *cmd,
    struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut shift: u32 = pmf_next32(cmd);
    let mut mask: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: write_reg16_slm(offset: %x, shift: %x, mask: %x\n",
    offset, shift, mask);
    PMF_PARSE_CALL(write_reg16_slm, cmd, h, offset, shift, mask);
    }
    static int pmf_parser_write_reg8_slm(struct pmf_cmd *cmd,
    struct pmf_handlers *h)
    {
    let mut offset: u32 = pmf_next32(cmd);
    let mut shift: u32 = pmf_next32(cmd);
    let mut mask: u32 = pmf_next32(cmd);
    LOG_PARSE("pmf: write_reg8_slm(offset: %x, shift: %x, mask: %x\n",
    offset, shift, mask);
    PMF_PARSE_CALL(write_reg8_slm, cmd, h, offset, shift, mask);
    }
    static int pmf_parser_mask_and_compare(struct pmf_cmd *cmd,
    struct pmf_handlers *h)
    {
    let mut bytes: u32 = pmf_next32(cmd);
    const void *maskblob = pmf_next_blob(cmd, bytes);
    const void *valuesblob = pmf_next_blob(cmd, bytes);
    LOG_PARSE("pmf: mask_and_compare(length: %ud ...\n", bytes);
    LOG_BLOB("pmf:   mask data: \n", maskblob, bytes);
    LOG_BLOB("pmf:   values data: \n", valuesblob, bytes);
    PMF_PARSE_CALL(mask_and_compare, cmd, h,
    bytes, maskblob, valuesblob);
    }
    typedef int (*pmf_cmd_parser_t)(struct pmf_cmd *cmd, struct pmf_handlers *h);
    static pmf_cmd_parser_t pmf_parsers[PMF_CMD_COUNT] =
    {
    core::ptr::null_mut(),
    pmf_parser_write_gpio,
    pmf_parser_read_gpio,
    pmf_parser_write_reg32,
    pmf_parser_read_reg32,
    pmf_parser_write_reg16,
    pmf_parser_read_reg16,
    pmf_parser_write_reg8,
    pmf_parser_read_reg8,
    pmf_parser_delay,
    pmf_parser_wait_reg32,
    pmf_parser_wait_reg16,
    pmf_parser_wait_reg8,
    pmf_parser_read_i2c,
    pmf_parser_write_i2c,
    pmf_parser_rmw_i2c,
    core::ptr::null_mut(), /* Bogus command */
    core::ptr::null_mut(), /* Shift bytes right: NYI */
    core::ptr::null_mut(), /* Shift bytes left: NYI */
    pmf_parser_read_cfg,
    pmf_parser_write_cfg,
    pmf_parser_rmw_cfg,
    pmf_parser_read_i2c_sub,
    pmf_parser_write_i2c_sub,
    pmf_parser_set_i2c_mode,
    pmf_parser_rmw_i2c_sub,
    pmf_parser_read_reg32_msrx,
    pmf_parser_read_reg16_msrx,
    pmf_parser_read_reg8_msrx,
    pmf_parser_write_reg32_slm,
    pmf_parser_write_reg16_slm,
    pmf_parser_write_reg8_slm,
    pmf_parser_mask_and_compare,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_device {
    pub link: list_head,
    pub node: *mut device_node,
    pub handlers: *mut pmf_handlers,
    pub functions: list_head,
    pub ref: kref,
}

    static LIST_HEAD(pmf_devices);
    static DEFINE_SPINLOCK(pmf_lock);
    static DEFINE_MUTEX(pmf_irq_mutex);
#[no_mangle]
unsafe extern "C" fn pmf_release_device(kref: *mut kref) {
    static void pmf_release_device(struct kref *kref)
    {
    struct pmf_device *dev = container_of(kref, struct pmf_device, ref);
    kfree(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn pmf_put_device(dev: *mut pmf_device) {
    static inline void pmf_put_device(struct pmf_device *dev)
    {
    kref_put(&dev.ref, pmf_release_device);
    }
    static inline struct pmf_device *pmf_get_device(struct pmf_device *dev)
    {
    kref_get(&dev.ref);
    return dev;
    }
    static inline struct pmf_device *pmf_find_device(struct device_node *np)
    {
    struct pmf_device *dev;
    list_for_each_entry(dev, &pmf_devices, link) {
    if (dev.node == np)
    return pmf_get_device(dev);
    }
    return core::ptr::null_mut();
    }
    static int pmf_parse_one(struct pmf_function *func,
    struct pmf_handlers *handlers,
    void *instdata, struct pmf_args *args)
    {
    struct pmf_cmd cmd;
    u32 ccode;
    int count, rc;
    cmd.cmdptr		= func.data;
    cmd.cmdend		= func.data + func.length;
    cmd.func       		= func;
    cmd.instdata		= instdata;
    cmd.args		= args;
    cmd.error		= 0;
    LOG_PARSE("pmf: func %s, %d bytes, %s...\n",
    func.name, func.length,
    handlers ? "executing" : "parsing");
// One subcommand to parse for now
    count = 1;
    while(count-- && cmd.cmdptr < cmd.cmdend) {
// Get opcode
    ccode = pmf_next32(&cmd);
// Check if we are hitting a command list, fetch new count
    if (ccode == 0) {
    count = pmf_next32(&cmd) - 1;
    ccode = pmf_next32(&cmd);
    }
    if (cmd.error) {
    LOG_ERROR("pmf: parse error, not enough data\n");
    return -ENXIO;
    }
    if (ccode >= PMF_CMD_COUNT) {
    LOG_ERROR("pmf: command code %d unknown !\n", ccode);
    return -ENXIO;
    }
    if (pmf_parsers[ccode] == core::ptr::null_mut()) {
    LOG_ERROR("pmf: no parser for command %d !\n", ccode);
    return -ENXIO;
    }
    rc = pmf_parsers[ccode](&cmd, handlers);
    if (rc != 0) {
    LOG_ERROR("pmf: parser for command %d returned"
    " error %d\n", ccode, rc);
    return rc;
    }
    }
// We are doing an initial parse pass, we need to adjust the size
    if (handlers == core::ptr::null_mut())
    func.length = cmd.cmdptr - func.data;
    return 0;
    }
    static int pmf_add_function_prop(struct pmf_device *dev, void *driverdata,
    const char *name, u32 *data,
    unsigned int length)
    {
    let mut count: c_int = 0;
    struct pmf_function *func = core::ptr::null_mut();
    DBG("pmf: Adding functions for platform-do-%s\n", name);
    while (length >= 12) {
// Allocate a structure
    func = kzalloc_obj(*func);
    if (func == core::ptr::null_mut())
    goto bail;
    kref_init(&func.ref);
    INIT_LIST_HEAD(&func.irq_clients);
    func.node = dev.node;
    func.driver_data = driverdata;
    func.name = name;
    func.phandle = data[0];
    func.flags = data[1];
    data += 2;
    length -= 8;
    func.data = data;
    func.length = length;
    func.dev = dev;
    DBG("pmf: idx %d: flags=%08x, phandle=%08x "
    " %d bytes remaining, parsing...\n",
    count+1, func.flags, func.phandle, length);
    if (pmf_parse_one(func, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut())) {
    kfree(func);
    goto bail;
    }
    length -= func.length;
    data = (u32 *)(((u8 *)data) + func.length);
    list_add(&func.link, &dev.functions);
    pmf_get_device(dev);
    count++;
    }
    bail:
    DBG("pmf: Added %d functions\n", count);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn pmf_add_functions(dev: *mut pmf_device, driverdata: *mut c_void) -> c_int {
    static int pmf_add_functions(struct pmf_device *dev, void *driverdata)
    {
    struct property *pp;

    let mut plen: c_int = strlen(PP_PREFIX);
    let mut count: c_int = 0;
    for_each_property_of_node(dev.node, pp) {
    const char *name;
    if (strncmp(pp.name, PP_PREFIX, plen) != 0)
    continue;
    name = pp.name + plen;
    if (strlen(name) && pp.length >= 12)
    count += pmf_add_function_prop(dev, driverdata, name,
    pp.value, pp.length);
    }
    return count;
    }
    int pmf_register_driver(struct device_node *np,
    struct pmf_handlers *handlers,
    void *driverdata)
    {
    struct pmf_device *dev;
    unsigned long flags;
    let mut rc: c_int = 0;
    if (handlers == core::ptr::null_mut())
    return -EINVAL;
    DBG("pmf: registering driver for node %pOF\n", np);
    spin_lock_irqsave(&pmf_lock, flags);
    dev = pmf_find_device(np);
    spin_unlock_irqrestore(&pmf_lock, flags);
    if (dev != core::ptr::null_mut()) {
    DBG("pmf: already there !\n");
    pmf_put_device(dev);
    return -EBUSY;
    }
    dev = kzalloc_obj(*dev);
    if (dev == core::ptr::null_mut()) {
    DBG("pmf: no memory !\n");
    return -ENOMEM;
    }
    kref_init(&dev.ref);
    dev.node = of_node_get(np);
    dev.handlers = handlers;
    INIT_LIST_HEAD(&dev.functions);
    rc = pmf_add_functions(dev, driverdata);
    if (rc == 0) {
    DBG("pmf: no functions, disposing.. \n");
    of_node_put(np);
    kfree(dev);
    return -ENODEV;
    }
    spin_lock_irqsave(&pmf_lock, flags);
    list_add(&dev.link, &pmf_devices);
    spin_unlock_irqrestore(&pmf_lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(pmf_register_driver);
    struct pmf_function *pmf_get_function(struct pmf_function *func)
    {
    if (!try_module_get(func.dev.handlers.owner))
    return core::ptr::null_mut();
    kref_get(&func.ref);
    return func;
    }
    EXPORT_SYMBOL_GPL(pmf_get_function);
#[no_mangle]
unsafe extern "C" fn pmf_release_function(kref: *mut kref) {
    static void pmf_release_function(struct kref *kref)
    {
    struct pmf_function *func =
    container_of(kref, struct pmf_function, ref);
    pmf_put_device(func.dev);
    kfree(func);
    }
#[no_mangle]
pub unsafe extern "C" fn __pmf_put_function(func: *mut pmf_function) {
    static inline void __pmf_put_function(struct pmf_function *func)
    {
    kref_put(&func.ref, pmf_release_function);
    }
#[no_mangle]
pub unsafe extern "C" fn pmf_put_function(func: *mut pmf_function) {
    void pmf_put_function(struct pmf_function *func)
    {
    if (func == core::ptr::null_mut())
    return;
    module_put(func.dev.handlers.owner);
    __pmf_put_function(func);
    }
    EXPORT_SYMBOL_GPL(pmf_put_function);
#[no_mangle]
pub unsafe extern "C" fn pmf_unregister_driver(np: *mut device_node) {
    void pmf_unregister_driver(struct device_node *np)
    {
    struct pmf_device *dev;
    unsigned long flags;
    DBG("pmf: unregistering driver for node %pOF\n", np);
    spin_lock_irqsave(&pmf_lock, flags);
    dev = pmf_find_device(np);
    if (dev == core::ptr::null_mut()) {
    DBG("pmf: not such driver !\n");
    spin_unlock_irqrestore(&pmf_lock, flags);
    return;
    }
    list_del(&dev.link);
    while(!list_empty(&dev.functions)) {
    struct pmf_function *func =
    list_entry(dev.functions.next, typeof(*func), link);
    list_del(&func.link);
    __pmf_put_function(func);
    }
    pmf_put_device(dev);
    spin_unlock_irqrestore(&pmf_lock, flags);
    }
    EXPORT_SYMBOL_GPL(pmf_unregister_driver);
    static struct pmf_function *__pmf_find_function(struct device_node *target,
    const char *name, u32 flags)
    {
    struct device_node *actor = of_node_get(target);
    struct pmf_device *dev;
    struct pmf_function *func, *result = core::ptr::null_mut();
    char fname[64];
    const u32 *prop;
    u32 ph;
//
// Look for a "platform-*" function reference. If we can't find
// one, then we fallback to a direct call attempt
//
    snprintf(fname, 63, "platform-%s", name);
    prop = of_get_property(target, fname, core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    goto find_it;
    ph = *prop;
    if (ph == 0)
    goto find_it;
//
// Ok, now try to find the actor. If we can't find it, we fail,
// there is no point in falling back there
//
    of_node_put(actor);
    actor = of_find_node_by_phandle(ph);
    if (actor == core::ptr::null_mut())
    return core::ptr::null_mut();
    find_it:
    dev = pmf_find_device(actor);
    if (dev == core::ptr::null_mut()) {
    result = core::ptr::null_mut();
    goto out;
    }
    list_for_each_entry(func, &dev.functions, link) {
    if (name && strcmp(name, func.name))
    continue;
    if (func.phandle && target.phandle != func.phandle)
    continue;
    if ((func.flags & flags) == 0)
    continue;
    result = func;
    break;
    }
    pmf_put_device(dev);
    out:
    of_node_put(actor);
    return result;
    }
    int pmf_register_irq_client(struct device_node *target,
    const char *name,
    struct pmf_irq_client *client)
    {
    struct pmf_function *func;
    unsigned long flags;
    spin_lock_irqsave(&pmf_lock, flags);
    func = __pmf_find_function(target, name, PMF_FLAGS_INT_GEN);
    if (func)
    func = pmf_get_function(func);
    spin_unlock_irqrestore(&pmf_lock, flags);
    if (func == core::ptr::null_mut())
    return -ENODEV;
// guard against manipulations of list
    mutex_lock(&pmf_irq_mutex);
    if (list_empty(&func.irq_clients))
    func.dev.handlers.irq_enable(func);
// guard against pmf_do_irq while changing list
    spin_lock_irqsave(&pmf_lock, flags);
    list_add(&client.link, &func.irq_clients);
    spin_unlock_irqrestore(&pmf_lock, flags);
    client.func = func;
    mutex_unlock(&pmf_irq_mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(pmf_register_irq_client);
#[no_mangle]
pub unsafe extern "C" fn pmf_unregister_irq_client(client: *mut pmf_irq_client) {
    void pmf_unregister_irq_client(struct pmf_irq_client *client)
    {
    struct pmf_function *func = client.func;
    unsigned long flags;
    BUG_ON(func == core::ptr::null_mut());
// guard against manipulations of list
    mutex_lock(&pmf_irq_mutex);
    client.func = core::ptr::null_mut();
// guard against pmf_do_irq while changing list
    spin_lock_irqsave(&pmf_lock, flags);
    list_del(&client.link);
    spin_unlock_irqrestore(&pmf_lock, flags);
    if (list_empty(&func.irq_clients))
    func.dev.handlers.irq_disable(func);
    mutex_unlock(&pmf_irq_mutex);
    pmf_put_function(func);
    }
    EXPORT_SYMBOL_GPL(pmf_unregister_irq_client);
#[no_mangle]
pub unsafe extern "C" fn pmf_do_irq(func: *mut pmf_function) {
    void pmf_do_irq(struct pmf_function *func)
    {
    unsigned long flags;
    struct pmf_irq_client *client;
// For now, using a spinlock over the whole function. Can be made
// to drop the lock using 2 lists if necessary
//
    spin_lock_irqsave(&pmf_lock, flags);
    list_for_each_entry(client, &func.irq_clients, link) {
    if (!try_module_get(client.owner))
    continue;
    client.handler(client.data);
    module_put(client.owner);
    }
    spin_unlock_irqrestore(&pmf_lock, flags);
    }
    EXPORT_SYMBOL_GPL(pmf_do_irq);
#[no_mangle]
pub unsafe extern "C" fn pmf_call_one(func: *mut pmf_function, args: *mut pmf_args) -> c_int {
    int pmf_call_one(struct pmf_function *func, struct pmf_args *args)
    {
    struct pmf_device *dev = func.dev;
    void *instdata = core::ptr::null_mut();
    let mut rc: c_int = 0;
    DBG(" ** pmf_call_one(%pOF/%s) **\n", dev.node, func.name);
    if (dev.handlers.begin)
    instdata = dev.handlers.begin(func, args);
    rc = pmf_parse_one(func, dev.handlers, instdata, args);
    if (dev.handlers.end)
    dev.handlers.end(func, instdata);
    return rc;
    }
    EXPORT_SYMBOL_GPL(pmf_call_one);
    int pmf_do_functions(struct device_node *np, const char *name,
    u32 phandle, u32 fflags, struct pmf_args *args)
    {
    struct pmf_device *dev;
    struct pmf_function *func, *tmp;
    unsigned long flags;
    let mut rc: c_int = -ENODEV;
    spin_lock_irqsave(&pmf_lock, flags);
    dev = pmf_find_device(np);
    if (dev == core::ptr::null_mut()) {
    spin_unlock_irqrestore(&pmf_lock, flags);
    return -ENODEV;
    }
    list_for_each_entry_safe(func, tmp, &dev.functions, link) {
    if (name && strcmp(name, func.name))
    continue;
    if (phandle && func.phandle && phandle != func.phandle)
    continue;
    if ((func.flags & fflags) == 0)
    continue;
    if (pmf_get_function(func) == core::ptr::null_mut())
    continue;
    spin_unlock_irqrestore(&pmf_lock, flags);
    rc = pmf_call_one(func, args);
    pmf_put_function(func);
    spin_lock_irqsave(&pmf_lock, flags);
    }
    pmf_put_device(dev);
    spin_unlock_irqrestore(&pmf_lock, flags);
    return rc;
    }
    EXPORT_SYMBOL_GPL(pmf_do_functions);
    struct pmf_function *pmf_find_function(struct device_node *target,
    const char *name)
    {
    struct pmf_function *func;
    unsigned long flags;
    spin_lock_irqsave(&pmf_lock, flags);
    func = __pmf_find_function(target, name, PMF_FLAGS_ON_DEMAND);
    if (func)
    func = pmf_get_function(func);
    spin_unlock_irqrestore(&pmf_lock, flags);
    return func;
    }
    EXPORT_SYMBOL_GPL(pmf_find_function);
    int pmf_call_function(struct device_node *target, const char *name,
    struct pmf_args *args)
    {
    struct pmf_function *func = pmf_find_function(target, name);
    int rc;
    if (func == core::ptr::null_mut())
    return -ENODEV;
    rc = pmf_call_one(func, args);
    pmf_put_function(func);
    return rc;
    }
    EXPORT_SYMBOL_GPL(pmf_call_function);
