//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utstrsuppt.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utstrsuppt - Support functions for string-to-integer conversion
//

    ACPI_MODULE_NAME("utstrsuppt")
// Local prototypes
    static acpi_status
    acpi_ut_insert_digit(u64 *accumulated_value, u32 base, int ascii_digit);
    static acpi_status
    acpi_ut_strtoul_multiply64(u64 multiplicand, u32 base, u64 *out_product);
    static acpi_status acpi_ut_strtoul_add64(u64 addend1, u32 digit, u64 *out_sum);
//
// FUNCTION:    acpi_ut_convert_octal_string
//
// PARAMETERS:  string                  - Null terminated input string
// return_value_ptr        - Where the converted value is returned
//
// RETURN:      Status and 64-bit converted integer
//
// DESCRIPTION: Performs a base 8 conversion of the input string to an
// integer value, either 32 or 64 bits.
//
// NOTE:        Maximum 64-bit unsigned octal value is 01777777777777777777777
// Maximum 32-bit unsigned octal value is 037777777777
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_convert_octal_string(string: *mut c_char, return_value_ptr: *mut u64) -> acpi_status {
    acpi_status acpi_ut_convert_octal_string(char *string, u64 *return_value_ptr)
    {
    let mut accumulated_value: u64 = 0;
    let mut status: acpi_status = AE_OK;
// Convert each ASCII byte in the input string
    while (*string) {
//
// Character must be ASCII 0-7, otherwise:
// 1) Runtime: terminate with no error, per the ACPI spec
// 2) Compiler: return an error
//
    if (!(ACPI_IS_OCTAL_DIGIT(*string))) {

    status = AE_BAD_OCTAL_CONSTANT;

    break;
    }
// Convert and insert this octal digit into the accumulator
    status = acpi_ut_insert_digit(&accumulated_value, 8, *string);
    if (ACPI_FAILURE(status)) {
    status = AE_OCTAL_OVERFLOW;
    break;
    }
    string++;
    }
// Always return the value that has been accumulated
// return_value_ptr = accumulated_value;
    return (status);
    }
//
// FUNCTION:    acpi_ut_convert_decimal_string
//
// PARAMETERS:  string                  - Null terminated input string
// return_value_ptr        - Where the converted value is returned
//
// RETURN:      Status and 64-bit converted integer
//
// DESCRIPTION: Performs a base 10 conversion of the input string to an
// integer value, either 32 or 64 bits.
//
// NOTE:        Maximum 64-bit unsigned decimal value is 18446744073709551615
// Maximum 32-bit unsigned decimal value is 4294967295
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_convert_decimal_string(string: *mut c_char, return_value_ptr: *mut u64) -> acpi_status {
    acpi_status acpi_ut_convert_decimal_string(char *string, u64 *return_value_ptr)
    {
    let mut accumulated_value: u64 = 0;
    let mut status: acpi_status = AE_OK;
// Convert each ASCII byte in the input string
    while (*string) {
//
// Character must be ASCII 0-9, otherwise:
// 1) Runtime: terminate with no error, per the ACPI spec
// 2) Compiler: return an error
//
    if (!isdigit((int)*string)) {

    status = AE_BAD_DECIMAL_CONSTANT;

    break;
    }
// Convert and insert this decimal digit into the accumulator
    status = acpi_ut_insert_digit(&accumulated_value, 10, *string);
    if (ACPI_FAILURE(status)) {
    status = AE_DECIMAL_OVERFLOW;
    break;
    }
    string++;
    }
// Always return the value that has been accumulated
// return_value_ptr = accumulated_value;
    return (status);
    }
//
// FUNCTION:    acpi_ut_convert_hex_string
//
// PARAMETERS:  string                  - Null terminated input string
// return_value_ptr        - Where the converted value is returned
//
// RETURN:      Status and 64-bit converted integer
//
// DESCRIPTION: Performs a base 16 conversion of the input string to an
// integer value, either 32 or 64 bits.
//
// NOTE:        Maximum 64-bit unsigned hex value is 0xFFFFFFFFFFFFFFFF
// Maximum 32-bit unsigned hex value is 0xFFFFFFFF
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_convert_hex_string(string: *mut c_char, return_value_ptr: *mut u64) -> acpi_status {
    acpi_status acpi_ut_convert_hex_string(char *string, u64 *return_value_ptr)
    {
    let mut accumulated_value: u64 = 0;
    let mut status: acpi_status = AE_OK;
// Convert each ASCII byte in the input string
    while (*string) {
//
// Character must be ASCII A-F, a-f, or 0-9, otherwise:
// 1) Runtime: terminate with no error, per the ACPI spec
// 2) Compiler: return an error
//
    if (!isxdigit((int)*string)) {

    status = AE_BAD_HEX_CONSTANT;

    break;
    }
// Convert and insert this hex digit into the accumulator
    status = acpi_ut_insert_digit(&accumulated_value, 16, *string);
    if (ACPI_FAILURE(status)) {
    status = AE_HEX_OVERFLOW;
    break;
    }
    string++;
    }
// Always return the value that has been accumulated
// return_value_ptr = accumulated_value;
    return (status);
    }
//
// FUNCTION:    acpi_ut_remove_leading_zeros
//
// PARAMETERS:  string                  - Pointer to input ASCII string
//
// RETURN:      Next character after any leading zeros. This character may be
// used by the caller to detect end-of-string.
//
// DESCRIPTION: Remove any leading zeros in the input string. Return the
// next character after the final ASCII zero to enable the caller
// to check for the end of the string (NULL terminator).
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_remove_leading_zeros(string: *mut c_char) -> c_char {
    char acpi_ut_remove_leading_zeros(char **string)
    {
    while (**string == ACPI_ASCII_ZERO) {
// string += 1;
    }
    return (**string);
    }
//
// FUNCTION:    acpi_ut_remove_whitespace
//
// PARAMETERS:  string                  - Pointer to input ASCII string
//
// RETURN:      Next character after any whitespace. This character may be
// used by the caller to detect end-of-string.
//
// DESCRIPTION: Remove any leading whitespace in the input string. Return the
// next character after the final ASCII zero to enable the caller
// to check for the end of the string (NULL terminator).
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_remove_whitespace(string: *mut c_char) -> c_char {
    char acpi_ut_remove_whitespace(char **string)
    {
    while (isspace((u8)**string)) {
// string += 1;
    }
    return (**string);
    }
//
// FUNCTION:    acpi_ut_detect_hex_prefix
//
// PARAMETERS:  string                  - Pointer to input ASCII string
//
// RETURN:      TRUE if a "0x" prefix was found at the start of the string
//
// DESCRIPTION: Detect and remove a hex "0x" prefix
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_detect_hex_prefix(string: *mut c_char) -> u8 {
    u8 acpi_ut_detect_hex_prefix(char **string)
    {
    char *initial_position = *string;
    acpi_ut_remove_hex_prefix(string);
    if (*string != initial_position) {
    return (TRUE);	/* String is past leading 0x */
    }
    return (FALSE);		/* Not a hex string */
    }
//
// FUNCTION:    acpi_ut_remove_hex_prefix
//
// PARAMETERS:  string                  - Pointer to input ASCII string
//
// RETURN:      none
//
// DESCRIPTION: Remove a hex "0x" prefix
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_remove_hex_prefix(string: *mut c_char) {
    void acpi_ut_remove_hex_prefix(char **string)
    {
    if ((**string == ACPI_ASCII_ZERO) &&
    (tolower((int)*(*string + 1)) == 'x')) {
// string += 2;	/* Go past the leading 0x
    }
    }
//
// FUNCTION:    acpi_ut_detect_octal_prefix
//
// PARAMETERS:  string                  - Pointer to input ASCII string
//
// RETURN:      True if an octal "0" prefix was found at the start of the
// string
//
// DESCRIPTION: Detect and remove an octal prefix (zero)
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_detect_octal_prefix(string: *mut c_char) -> u8 {
    u8 acpi_ut_detect_octal_prefix(char **string)
    {
    if (**string == ACPI_ASCII_ZERO) {
// string += 1;	/* Go past the leading 0
    return (TRUE);
    }
    return (FALSE);		/* Not an octal string */
    }
//
// FUNCTION:    acpi_ut_insert_digit
//
// PARAMETERS:  accumulated_value       - Current value of the integer value
// accumulator. The new value is
// returned here.
// base                    - Radix, either 8/10/16
// ascii_digit             - ASCII single digit to be inserted
//
// RETURN:      Status and result of the convert/insert operation. The only
// possible returned exception code is numeric overflow of
// either the multiply or add conversion operations.
//
// DESCRIPTION: Generic conversion and insertion function for all bases:
//
// 1) Multiply the current accumulated/converted value by the
// base in order to make room for the new character.
//
// 2) Convert the new character to binary and add it to the
// current accumulated value.
//
// Note: The only possible exception indicates an integer
// overflow (AE_NUMERIC_OVERFLOW)
//
    static acpi_status
    acpi_ut_insert_digit(u64 *accumulated_value, u32 base, int ascii_digit)
    {
    acpi_status status;
    u64 product;
// Make room in the accumulated value for the incoming digit
    status = acpi_ut_strtoul_multiply64(*accumulated_value, base, &product);
    if (ACPI_FAILURE(status)) {
    return (status);
    }
// Add in the new digit, and store the sum to the accumulated value
    status =
    acpi_ut_strtoul_add64(product,
    acpi_ut_ascii_char_to_hex(ascii_digit),
    accumulated_value);
    return (status);
    }
//
// FUNCTION:    acpi_ut_strtoul_multiply64
//
// PARAMETERS:  multiplicand            - Current accumulated converted integer
// base                    - Base/Radix
// out_product             - Where the product is returned
//
// RETURN:      Status and 64-bit product
//
// DESCRIPTION: Multiply two 64-bit values, with checking for 64-bit overflow as
// well as 32-bit overflow if necessary (if the current global
// integer width is 32).
//
    static acpi_status
    acpi_ut_strtoul_multiply64(u64 multiplicand, u32 base, u64 *out_product)
    {
    u64 product;
    u64 quotient;
// Exit if either operand is zero
// out_product = 0;
    if (!multiplicand || !base) {
    return (AE_OK);
    }
//
// Check for 64-bit overflow before the actual multiplication.
//
// Notes: 64-bit division is often not supported on 32-bit platforms
// (it requires a library function), Therefore ACPICA has a local
// 64-bit divide function. Also, Multiplier is currently only used
// as the radix (8/10/16), to the 64/32 divide will always work.
//
    acpi_ut_short_divide(ACPI_UINT64_MAX, base, &quotient, core::ptr::null_mut());
    if (multiplicand > quotient) {
    return (AE_NUMERIC_OVERFLOW);
    }
    product = multiplicand * base;
// Check for 32-bit overflow if necessary
    if ((acpi_gbl_integer_bit_width == 32) && (product > ACPI_UINT32_MAX)) {
    return (AE_NUMERIC_OVERFLOW);
    }
// out_product = product;
    return (AE_OK);
    }
//
// FUNCTION:    acpi_ut_strtoul_add64
//
// PARAMETERS:  addend1                 - Current accumulated converted integer
// digit                   - New hex value/char
// out_sum                 - Where sum is returned (Accumulator)
//
// RETURN:      Status and 64-bit sum
//
// DESCRIPTION: Add two 64-bit values, with checking for 64-bit overflow as
// well as 32-bit overflow if necessary (if the current global
// integer width is 32).
//
#[no_mangle]
unsafe extern "C" fn acpi_ut_strtoul_add64(addend1: u64, digit: u32, out_sum: *mut u64) -> acpi_status {
    static acpi_status acpi_ut_strtoul_add64(u64 addend1, u32 digit, u64 *out_sum)
    {
    u64 sum;
// Check for 64-bit overflow before the actual addition
    if ((addend1 > 0) && (digit > (ACPI_UINT64_MAX - addend1))) {
    return (AE_NUMERIC_OVERFLOW);
    }
    sum = addend1 + digit;
// Check for 32-bit overflow if necessary
    if ((acpi_gbl_integer_bit_width == 32) && (sum > ACPI_UINT32_MAX)) {
    return (AE_NUMERIC_OVERFLOW);
    }
// out_sum = sum;
    return (AE_OK);
    }
