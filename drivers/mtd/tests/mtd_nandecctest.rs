//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/tests/mtd_nandecctest.c
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
// Test the implementation for software ECC
//
// No actual MTD device is needed, So we don't need to warry about losing
// important data by human error.
//
// This covers possible patterns of corruption which can be reliably corrected
// or detected.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_test {
    pub name: *const c_char,
    pub size_t): *const *const *const *const *const *const void (prepare)(void , void , void , void ,,
    pub size_t): *const *const *const *const *const int (verify)(void , void , void ,,
}

//
// The reason for this __change_bit_le() instead of __change_bit() is to inject
// bit error properly within the region which is not a multiple of
// sizeof(unsigned long) on big-endian systems
//

    __change_bit((nr) ^ ((BITS_PER_LONG - 1) & ~0x7), addr)

    static void single_bit_error_data(void *error_data, void *correct_data,
    size_t size)
    {
    let mut offset: c_uint = get_random_u32_below(size * BITS_PER_BYTE);
    memcpy(error_data, correct_data, size);
    __change_bit_le(offset, error_data);
    }
    static void double_bit_error_data(void *error_data, void *correct_data,
    size_t size)
    {
    unsigned int offset[2];
    offset[0] = get_random_u32_below(size * BITS_PER_BYTE);
    do {
    offset[1] = get_random_u32_below(size * BITS_PER_BYTE);
    } while (offset[0] == offset[1]);
    memcpy(error_data, correct_data, size);
    __change_bit_le(offset[0], error_data);
    __change_bit_le(offset[1], error_data);
    }
#[no_mangle]
unsafe extern "C" fn random_ecc_bit(size: usize) -> c_uint {
    static unsigned int random_ecc_bit(size_t size)
    {
    let mut offset: c_uint = get_random_u32_below(3 * BITS_PER_BYTE);
    if (size == 256) {
//
// Don't inject a bit error into the insignificant bits (16th
// and 17th bit) in ECC code for 256 byte data block
//
    while (offset == 16 || offset == 17)
    offset = get_random_u32_below(3 * BITS_PER_BYTE);
    }
    return offset;
    }
    static void single_bit_error_ecc(void *error_ecc, void *correct_ecc,
    size_t size)
    {
    let mut offset: c_uint = random_ecc_bit(size);
    memcpy(error_ecc, correct_ecc, 3);
    __change_bit_le(offset, error_ecc);
    }
    static void double_bit_error_ecc(void *error_ecc, void *correct_ecc,
    size_t size)
    {
    unsigned int offset[2];
    offset[0] = random_ecc_bit(size);
    do {
    offset[1] = random_ecc_bit(size);
    } while (offset[0] == offset[1]);
    memcpy(error_ecc, correct_ecc, 3);
    __change_bit_le(offset[0], error_ecc);
    __change_bit_le(offset[1], error_ecc);
    }
    static void no_bit_error(void *error_data, void *error_ecc,
    void *correct_data, void *correct_ecc, const size_t size)
    {
    memcpy(error_data, correct_data, size);
    memcpy(error_ecc, correct_ecc, 3);
    }
    static int no_bit_error_verify(void *error_data, void *error_ecc,
    void *correct_data, const size_t size)
    {
    let mut sm_order: bool = IS_ENABLED(CONFIG_MTD_NAND_ECC_SW_HAMMING_SMC);
    unsigned char calc_ecc[3];
    int ret;
    ecc_sw_hamming_calculate(error_data, size, calc_ecc, sm_order);
    ret = ecc_sw_hamming_correct(error_data, error_ecc, calc_ecc, size,
    sm_order);
    if (ret == 0 && !memcmp(correct_data, error_data, size))
    return 0;
    return -EINVAL;
    }
    static void single_bit_error_in_data(void *error_data, void *error_ecc,
    void *correct_data, void *correct_ecc, const size_t size)
    {
    single_bit_error_data(error_data, correct_data, size);
    memcpy(error_ecc, correct_ecc, 3);
    }
    static void single_bit_error_in_ecc(void *error_data, void *error_ecc,
    void *correct_data, void *correct_ecc, const size_t size)
    {
    memcpy(error_data, correct_data, size);
    single_bit_error_ecc(error_ecc, correct_ecc, size);
    }
    static int single_bit_error_correct(void *error_data, void *error_ecc,
    void *correct_data, const size_t size)
    {
    let mut sm_order: bool = IS_ENABLED(CONFIG_MTD_NAND_ECC_SW_HAMMING_SMC);
    unsigned char calc_ecc[3];
    int ret;
    ecc_sw_hamming_calculate(error_data, size, calc_ecc, sm_order);
    ret = ecc_sw_hamming_correct(error_data, error_ecc, calc_ecc, size,
    sm_order);
    if (ret == 1 && !memcmp(correct_data, error_data, size))
    return 0;
    return -EINVAL;
    }
    static void double_bit_error_in_data(void *error_data, void *error_ecc,
    void *correct_data, void *correct_ecc, const size_t size)
    {
    double_bit_error_data(error_data, correct_data, size);
    memcpy(error_ecc, correct_ecc, 3);
    }
    static void single_bit_error_in_data_and_ecc(void *error_data, void *error_ecc,
    void *correct_data, void *correct_ecc, const size_t size)
    {
    single_bit_error_data(error_data, correct_data, size);
    single_bit_error_ecc(error_ecc, correct_ecc, size);
    }
    static void double_bit_error_in_ecc(void *error_data, void *error_ecc,
    void *correct_data, void *correct_ecc, const size_t size)
    {
    memcpy(error_data, correct_data, size);
    double_bit_error_ecc(error_ecc, correct_ecc, size);
    }
    static int double_bit_error_detect(void *error_data, void *error_ecc,
    void *correct_data, const size_t size)
    {
    let mut sm_order: bool = IS_ENABLED(CONFIG_MTD_NAND_ECC_SW_HAMMING_SMC);
    unsigned char calc_ecc[3];
    int ret;
    ecc_sw_hamming_calculate(error_data, size, calc_ecc, sm_order);
    ret = ecc_sw_hamming_correct(error_data, error_ecc, calc_ecc, size,
    sm_order);
    return (ret == -EBADMSG) ? 0 : -EINVAL;
    }
    static const struct nand_ecc_test nand_ecc_test[] = {
    {
    .name = "no-bit-error",
    .prepare = no_bit_error,
    .verify = no_bit_error_verify,
    },
    {
    .name = "single-bit-error-in-data-correct",
    .prepare = single_bit_error_in_data,
    .verify = single_bit_error_correct,
    },
    {
    .name = "single-bit-error-in-ecc-correct",
    .prepare = single_bit_error_in_ecc,
    .verify = single_bit_error_correct,
    },
    {
    .name = "double-bit-error-in-data-detect",
    .prepare = double_bit_error_in_data,
    .verify = double_bit_error_detect,
    },
    {
    .name = "single-bit-error-in-data-and-ecc-detect",
    .prepare = single_bit_error_in_data_and_ecc,
    .verify = double_bit_error_detect,
    },
    {
    .name = "double-bit-error-in-ecc-detect",
    .prepare = double_bit_error_in_ecc,
    .verify = double_bit_error_detect,
    },
    };
    static void dump_data_ecc(void *error_data, void *error_ecc, void *correct_data,
    void *correct_ecc, const size_t size)
    {
    pr_info("hexdump of error data:\n");
    print_hex_dump(KERN_INFO, "", DUMP_PREFIX_OFFSET, 16, 4,
    error_data, size, false);
    print_hex_dump(KERN_INFO, "hexdump of error ecc: ",
    DUMP_PREFIX_NONE, 16, 1, error_ecc, 3, false);
    pr_info("hexdump of correct data:\n");
    print_hex_dump(KERN_INFO, "", DUMP_PREFIX_OFFSET, 16, 4,
    correct_data, size, false);
    print_hex_dump(KERN_INFO, "hexdump of correct ecc: ",
    DUMP_PREFIX_NONE, 16, 1, correct_ecc, 3, false);
    }
#[no_mangle]
unsafe extern "C" fn nand_ecc_test_run(size: usize) -> c_int {
    static int nand_ecc_test_run(const size_t size)
    {
    let mut sm_order: bool = IS_ENABLED(CONFIG_MTD_NAND_ECC_SW_HAMMING_SMC);
    int i;
    let mut err: c_int = 0;
    void *error_data;
    void *error_ecc;
    void *correct_data;
    void *correct_ecc;
    error_data = kmalloc(size, GFP_KERNEL);
    error_ecc = kmalloc(3, GFP_KERNEL);
    correct_data = kmalloc(size, GFP_KERNEL);
    correct_ecc = kmalloc(3, GFP_KERNEL);
    if (!error_data || !error_ecc || !correct_data || !correct_ecc) {
    err = -ENOMEM;
    goto error;
    }
    get_random_bytes(correct_data, size);
    ecc_sw_hamming_calculate(correct_data, size, correct_ecc, sm_order);
    for (i = 0; i < ARRAY_SIZE(nand_ecc_test); i++) {
    nand_ecc_test[i].prepare(error_data, error_ecc,
    correct_data, correct_ecc, size);
    err = nand_ecc_test[i].verify(error_data, error_ecc,
    correct_data, size);
    if (err) {
    pr_err("not ok - %s-%zd\n",
    nand_ecc_test[i].name, size);
    dump_data_ecc(error_data, error_ecc,
    correct_data, correct_ecc, size);
    break;
    }
    pr_info("ok - %s-%zd\n",
    nand_ecc_test[i].name, size);
    err = mtdtest_relax();
    if (err)
    break;
    }
    error:
    kfree(error_data);
    kfree(error_ecc);
    kfree(correct_data);
    kfree(correct_ecc);
    return err;
    }

#[no_mangle]
unsafe extern "C" fn nand_ecc_test_run(size: usize) -> c_int {
    static int nand_ecc_test_run(const size_t size)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ecc_test_init() -> int __init {
    static int __init ecc_test_init(void)
    {
    int err;
    err = nand_ecc_test_run(256);
    if (err)
    return err;
    return nand_ecc_test_run(512);
    }
#[no_mangle]
unsafe extern "C" fn ecc_test_exit() -> void __exit {
    static void __exit ecc_test_exit(void)
    {
    }
    module_init(ecc_test_init);
    module_exit(ecc_test_exit);
    MODULE_DESCRIPTION("NAND ECC function test module");
    MODULE_AUTHOR("Akinobu Mita");
    MODULE_LICENSE("GPL");
