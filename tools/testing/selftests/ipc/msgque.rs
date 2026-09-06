//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ipc/msgque.c
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
// Macro flag: #define _GNU_SOURCE

pub const MAX_MSG_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg1 {
    pub msize: c_int,
    pub mtype: c_long,
    pub mtext: [c_char; MAX_MSG_SIZE],
}

pub const MSG_TYPE: c_int = 1;

pub const ANOTHER_MSG_TYPE: c_int = 26538;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgque_data {
    pub key: key_t,
    pub msq_id: c_int,
    pub qbytes: c_int,
    pub qnum: c_int,
    pub mode: c_int,
    pub messages: *mut msg1,
}

#[no_mangle]
pub unsafe extern "C" fn restore_queue(msgque: *mut msgque_data) -> c_int {
    int restore_queue(struct msgque_data *msgque)
    {
    int fd, ret, id, i;
    char buf[32];
    fd = open("/proc/sys/kernel/msg_next_id", O_WRONLY);
    if (fd == -1) {
    ksft_test_result_fail("Failed to open /proc/sys/kernel/msg_next_id\n");
    return -errno;
    }
    sprintf(buf, "%d", msgque.msq_id);
    ret = write(fd, buf, strlen(buf));
    if (ret != strlen(buf)) {
    ksft_test_result_fail("Failed to write to /proc/sys/kernel/msg_next_id\n");
    return -errno;
    }
    id = msgget(msgque.key, msgque.mode | IPC_CREAT | IPC_EXCL);
    if (id == -1) {
    ksft_test_result_fail("Failed to create queue\n");
    return -errno;
    }
    if (id != msgque.msq_id) {
    ksft_test_result_fail("Restored queue has wrong id (%d instead of %d)\n"
    , id, msgque.msq_id);
    ret = -EFAULT;
    goto destroy;
    }
    for (i = 0; i < msgque.qnum; i++) {
    if (msgsnd(msgque.msq_id, &msgque.messages[i].mtype,
    msgque.messages[i].msize, IPC_NOWAIT) != 0) {
    ksft_test_result_fail("msgsnd failed (%m)\n");
    ret = -errno;
    goto destroy;
    }
    }
    return 0;
    destroy:
    if (msgctl(id, IPC_RMID, core::ptr::null_mut()))
    printf("Failed to destroy queue: %d\n", -errno);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn check_and_destroy_queue(msgque: *mut msgque_data) -> c_int {
    int check_and_destroy_queue(struct msgque_data *msgque)
    {
    struct msg1 message;
    let mut cnt: c_int = 0, ret;
    while (1) {
    ret = msgrcv(msgque.msq_id, &message.mtype, MAX_MSG_SIZE,
    0, IPC_NOWAIT);
    if (ret < 0) {
    if (errno == ENOMSG)
    break;
    ksft_test_result_fail("Failed to read IPC message: %m\n");
    ret = -errno;
    goto err;
    }
    if (ret != msgque.messages[cnt].msize) {
    ksft_test_result_fail("Wrong message size: %d (expected %d)\n", ret, msgque.messages[cnt].msize);
    ret = -EINVAL;
    goto err;
    }
    if (message.mtype != msgque.messages[cnt].mtype) {
    ksft_test_result_fail("Wrong message type\n");
    ret = -EINVAL;
    goto err;
    }
    if (memcmp(message.mtext, msgque.messages[cnt].mtext, ret)) {
    ksft_test_result_fail("Wrong message content\n");
    ret = -EINVAL;
    goto err;
    }
    cnt++;
    }
    if (cnt != msgque.qnum) {
    ksft_test_result_fail("Wrong message number\n");
    ret = -EINVAL;
    goto err;
    }
    ret = 0;
    err:
    if (msgctl(msgque.msq_id, IPC_RMID, core::ptr::null_mut())) {
    printf("Failed to destroy queue: %d\n", -errno);
    return -errno;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dump_queue(msgque: *mut msgque_data) -> c_int {
    int dump_queue(struct msgque_data *msgque)
    {
    struct msqid_ds ds;
    int kern_id;
    int i, ret;
    for (kern_id = 0; kern_id < 256; kern_id++) {
    ret = msgctl(kern_id, MSG_STAT, &ds);
    if (ret < 0) {
    if (errno == EINVAL)
    continue;
    ksft_test_result_fail("Failed to get stats for IPC queue with id %d\n",
    kern_id);
    return -errno;
    }
    if (ret == msgque.msq_id)
    break;
    }
    msgque.messages = malloc(sizeof(struct msg1) * ds.msg_qnum);
    if (msgque.messages == core::ptr::null_mut()) {
    ksft_test_result_fail("Failed to get stats for IPC queue\n");
    return -ENOMEM;
    }
    msgque.qnum = ds.msg_qnum;
    msgque.mode = ds.msg_perm.mode;
    msgque.qbytes = ds.msg_qbytes;
    for (i = 0; i < msgque.qnum; i++) {
    ret = msgrcv(msgque.msq_id, &msgque.messages[i].mtype,
    MAX_MSG_SIZE, i, IPC_NOWAIT | MSG_COPY);
    if (ret < 0) {
    if (errno == EOPNOTSUPP)
    ksft_exit_skip("MSG_COPY not supported\n");
    ksft_test_result_fail("Failed to copy IPC message: %m (%d)\n", errno);
    return -errno;
    }
    msgque.messages[i].msize = ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fill_msgque(msgque: *mut msgque_data) -> c_int {
    int fill_msgque(struct msgque_data *msgque)
    {
    struct msg1 msgbuf;
    msgbuf.mtype = MSG_TYPE;
    memcpy(msgbuf.mtext, TEST_STRING, sizeof(TEST_STRING));
    if (msgsnd(msgque.msq_id, &msgbuf.mtype, sizeof(TEST_STRING),
    IPC_NOWAIT) != 0) {
    ksft_test_result_fail("First message send failed (%m)\n");
    return -errno;
    }
    msgbuf.mtype = ANOTHER_MSG_TYPE;
    memcpy(msgbuf.mtext, ANOTHER_TEST_STRING, sizeof(ANOTHER_TEST_STRING));
    if (msgsnd(msgque.msq_id, &msgbuf.mtype, sizeof(ANOTHER_TEST_STRING),
    IPC_NOWAIT) != 0) {
    ksft_test_result_fail("Second message send failed (%m)\n");
    return -errno;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int err;
    struct msgque_data msgque;
    if (getuid() != 0)
    ksft_exit_skip("Please run the test as root - Exiting.\n");
    msgque.key = ftok(argv[0], 822155650);
    if (msgque.key == -1) {
    ksft_test_result_fail("Can't make key: %d\n", -errno);
    ksft_exit_fail();
    }
    msgque.msq_id = msgget(msgque.key, IPC_CREAT | IPC_EXCL | 0666);
    if (msgque.msq_id == -1) {
    err = -errno;
    ksft_test_result_fail("Can't create queue: %d\n", err);
    goto err_out;
    }
    err = fill_msgque(&msgque);
    if (err) {
    ksft_test_result_fail("Failed to fill queue: %d\n", err);
    goto err_destroy;
    }
    err = dump_queue(&msgque);
    if (err) {
    ksft_test_result_fail("Failed to dump queue: %d\n", err);
    goto err_destroy;
    }
    err = check_and_destroy_queue(&msgque);
    if (err) {
    ksft_test_result_fail("Failed to check and destroy queue: %d\n", err);
    goto err_out;
    }
    err = restore_queue(&msgque);
    if (err) {
    ksft_test_result_fail("Failed to restore queue: %d\n", err);
    goto err_destroy;
    }
    err = check_and_destroy_queue(&msgque);
    if (err) {
    ksft_test_result_fail("Failed to test queue: %d\n", err);
    goto err_out;
    }
    ksft_exit_pass();
    err_destroy:
    if (msgctl(msgque.msq_id, IPC_RMID, core::ptr::null_mut())) {
    printf("Failed to destroy queue: %d\n", -errno);
    ksft_exit_fail();
    }
    err_out:
    ksft_exit_fail();
    }
