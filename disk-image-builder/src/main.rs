use std::path::{Path, PathBuf};
use std::process::{exit, Command, ExitStatus};
use std::time::Duration;

use bootloader_locator::locate_bootloader;
use locate_cargo_manifest::locate_manifest;
use runner_utils::{binary_kind, BinaryKind};

const HEADLESS_ARGS: &[&str] = &[
    "-device",
    "isa-debug-exit,iobase=0xf4,iosize=0x04",
    "-display",
    "none",
];
const TEST_TIMEOUT: Duration = Duration::from_secs(10);

fn main() {
    let mut args = std::env::args().skip(1);

    let kernel_binary_path = {
        let path = PathBuf::from(args.next().unwrap());
        path.canonicalize().unwrap()
    };

    let launch_qemu = if let Some(arg) = args.next() {
        arg == "--launch-qemu"
    } else {
        false
    };

    let disk_image = create_disk_images(&kernel_binary_path);

    if launch_qemu {
        let kind = binary_kind(&kernel_binary_path);
        run_qemu(&disk_image, kind);
    }
}

fn create_disk_images(kernel_binary_path: &Path) -> PathBuf {
    let bootloader_manifest_path = locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_manifest().unwrap();

    let mut build_cmd = make_build_cmd(
        &bootloader_manifest_path,
        &kernel_manifest_path,
        kernel_binary_path,
    );

    if !build_cmd.status().unwrap().success() {
        panic!("failed to build disk image");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();

    kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-bios-{}.img", kernel_binary_name))
}

fn make_build_cmd(
    bootloader_manifest_path: &Path,
    kernel_manifest_path: &Path,
    kernel_binary_path: &Path,
) -> Command {
    let target_dir = kernel_manifest_path.parent().unwrap().join("target");
    let out_dir = kernel_binary_path.parent().unwrap();

    let mut cmd = Command::new(env!("CARGO"));
    cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    cmd.arg("builder");
    cmd.arg("--kernel-manifest").arg(&kernel_manifest_path);
    cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    cmd.arg("--target-dir").arg(target_dir);
    cmd.arg("--out-dir").arg(out_dir);
    cmd
}

fn run_qemu(disk_image: &Path, kind: BinaryKind) {
    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd.arg("-serial").arg("stdio");
    run_cmd
        .arg("-drive")
        .arg(format!("format=raw,file={}", disk_image.display()));
    run_cmd.arg("--no-reboot");

    if kind.is_test() {
        run_cmd.args(HEADLESS_ARGS);
        match run_test_command(run_cmd).code() {
            Some(33) => {}
            Some(error_code) => panic!("test run failed with error code {}", error_code),
            None => panic!("test run was terminated by a signal"),
        }
    } else {
        let status = run_cmd.status().unwrap();
        if !status.success() {
            exit(status.code().unwrap_or(-1));
        }
    }
}

fn run_test_command(mut cmd: Command) -> ExitStatus {
    runner_utils::run_with_timeout(&mut cmd, TEST_TIMEOUT).unwrap()
}
