//! Automatically rewritten from C to Rust
//! Source: samples/auxdisplay/cfag12864b-example.c
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
// Filename: cfag12864b-example.c
// Version: 0.1.0
// Description: cfag12864b LCD userspace example program
//
// Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
// Date: 2006-10-31
//
// ------------------------
// start of cfag12864b code
// ------------------------
//

    CFAG12864B_BPB + (x) / CFAG12864B_BPB)

    (y) < CFAG12864B_HEIGHT)

    int cfag12864b_fd;
    unsigned char * cfag12864b_mem;
    unsigned char cfag12864b_buffer[CFAG12864B_SIZE];
//
// init a cfag12864b framebuffer device
//
// No error:       return = 0
// Unable to open: return = -1
// Unable to mmap: return = -2
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_init(path: *mut c_char) -> c_int {
    static int cfag12864b_init(char *path)
    {
    cfag12864b_fd = open(path, O_RDWR);
    if (cfag12864b_fd == -1)
    return -1;
    cfag12864b_mem = mmap(0, CFAG12864B_SIZE, PROT_READ | PROT_WRITE,
    MAP_SHARED, cfag12864b_fd, 0);
    if (cfag12864b_mem == MAP_FAILED) {
    close(cfag12864b_fd);
    return -2;
    }
    return 0;
    }
//
// exit a cfag12864b framebuffer device
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_exit() {
    static void cfag12864b_exit(void)
    {
    munmap(cfag12864b_mem, CFAG12864B_SIZE);
    close(cfag12864b_fd);
    }
//
// set (x, y) pixel
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_set(x: c_uchar, y: c_uchar) {
    static void cfag12864b_set(unsigned char x, unsigned char y)
    {
    if (CFAG12864B_CHECK(x, y))
    cfag12864b_buffer[CFAG12864B_ADDRESS(x, y)] |=
    CFAG12864B_BIT(x % CFAG12864B_BPB);
    }
//
// unset (x, y) pixel
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_unset(x: c_uchar, y: c_uchar) {
    static void cfag12864b_unset(unsigned char x, unsigned char y)
    {
    if (CFAG12864B_CHECK(x, y))
    cfag12864b_buffer[CFAG12864B_ADDRESS(x, y)] &=
    ~CFAG12864B_BIT(x % CFAG12864B_BPB);
    }
//
// is set (x, y) pixel?
//
// Pixel off: return = 0
// Pixel on:  return = 1
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_isset(x: c_uchar, y: c_uchar) -> c_uchar {
    static unsigned char cfag12864b_isset(unsigned char x, unsigned char y)
    {
    if (CFAG12864B_CHECK(x, y))
    if (cfag12864b_buffer[CFAG12864B_ADDRESS(x, y)] &
    CFAG12864B_BIT(x % CFAG12864B_BPB))
    return 1;
    return 0;
    }
//
// not (x, y) pixel
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_not(x: c_uchar, y: c_uchar) {
    static void cfag12864b_not(unsigned char x, unsigned char y)
    {
    if (cfag12864b_isset(x, y))
    cfag12864b_unset(x, y);
    else
    cfag12864b_set(x, y);
    }
//
// fill (set all pixels)
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_fill() {
    static void cfag12864b_fill(void)
    {
    unsigned short i;
    for (i = 0; i < CFAG12864B_SIZE; i++)
    cfag12864b_buffer[i] = 0xFF;
    }
//
// clear (unset all pixels)
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_clear() {
    static void cfag12864b_clear(void)
    {
    unsigned short i;
    for (i = 0; i < CFAG12864B_SIZE; i++)
    cfag12864b_buffer[i] = 0;
    }
//
// format a [128*64] matrix
//
// Pixel off: src[i] = 0
// Pixel on:  src[i] > 0
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_format(matrix: *mut *mut c_uchar) {
    static void cfag12864b_format(unsigned char * matrix)
    {
    unsigned char i, j, n;
    for (i = 0; i < CFAG12864B_HEIGHT; i++)
    for (j = 0; j < CFAG12864B_WIDTH / CFAG12864B_BPB; j++) {
    cfag12864b_buffer[i * CFAG12864B_WIDTH / CFAG12864B_BPB +
    j] = 0;
    for (n = 0; n < CFAG12864B_BPB; n++)
    if (matrix[i * CFAG12864B_WIDTH +
    j * CFAG12864B_BPB + n])
    cfag12864b_buffer[i * CFAG12864B_WIDTH /
    CFAG12864B_BPB + j] |=
    CFAG12864B_BIT(n);
    }
    }
//
// blit buffer to lcd
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_blit() {
    static void cfag12864b_blit(void)
    {
    memcpy(cfag12864b_mem, cfag12864b_buffer, CFAG12864B_SIZE);
    }
//
// ----------------------
// end of cfag12864b code
// ----------------------
//

pub const EXAMPLES: c_int = 6;
#[no_mangle]
unsafe extern "C" fn example(n: c_uchar) {
    static void example(unsigned char n)
    {
    unsigned short i, j;
    unsigned char matrix[CFAG12864B_WIDTH * CFAG12864B_HEIGHT];
    if (n > EXAMPLES)
    return;
    printf("Example %i/%i - ", n, EXAMPLES);
    switch (n) {
    case 1:
    printf("Draw points setting bits");
    cfag12864b_clear();
    for (i = 0; i < CFAG12864B_WIDTH; i += 2)
    for (j = 0; j < CFAG12864B_HEIGHT; j += 2)
    cfag12864b_set(i, j);
    break;
    case 2:
    printf("Clear the LCD");
    cfag12864b_clear();
    break;
    case 3:
    printf("Draw rows formatting a [128*64] matrix");
    memset(matrix, 0, CFAG12864B_WIDTH * CFAG12864B_HEIGHT);
    for (i = 0; i < CFAG12864B_WIDTH; i++)
    for (j = 0; j < CFAG12864B_HEIGHT; j += 2)
    matrix[j * CFAG12864B_WIDTH + i] = 1;
    cfag12864b_format(matrix);
    break;
    case 4:
    printf("Fill the lcd");
    cfag12864b_fill();
    break;
    case 5:
    printf("Draw columns unsetting bits");
    for (i = 0; i < CFAG12864B_WIDTH; i += 2)
    for (j = 0; j < CFAG12864B_HEIGHT; j++)
    cfag12864b_unset(i, j);
    break;
    case 6:
    printf("Do negative not-ing all bits");
    for (i = 0; i < CFAG12864B_WIDTH; i++)
    for (j = 0; j < CFAG12864B_HEIGHT; j ++)
    cfag12864b_not(i, j);
    break;
    }
    puts(" - [Press Enter]");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    unsigned char n;
    if (argc != 2) {
    printf(
    "Syntax:  %s fbdev\n"
    "Usually: /dev/fb0, /dev/fb1...\n", argv[0]);
    return -1;
    }
    if (cfag12864b_init(argv[1])) {
    printf("Can't init %s fbdev\n", argv[1]);
    return -2;
    }
    for (n = 1; n <= EXAMPLES; n++) {
    example(n);
    cfag12864b_blit();
    while (getchar() != '\n');
    }
    cfag12864b_exit();
    return 0;
    }
