use crate::x86::hlt;
use crate::x86::write_io_port_u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x1, // QEMU will exit with status 3
    Failure = 0x2, // QEMU will exit with status 5
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    // QEMU では、0xF4 ポートに値を書き込むことで終了コードを設定します。
    // その後、0xF4 ポートに 0 を書き込むことで QEMU を終了します。
    // https://github.com/qemu/qemu/blob/master/hw/misc/debugexit.c
    write_io_port_u8(0xf4, exit_code as u8);
    loop {
        hlt()
    }
}
