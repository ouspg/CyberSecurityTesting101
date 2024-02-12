import argparse
import subprocess

def ping_device(target):
    command = f"ping -c 4 {target}"
    try:
        output = subprocess.check_output(command, shell=True, stderr=subprocess.STDOUT)
        print(output.decode())
    except subprocess.CalledProcessError as e:
        print(f"Failed to ping {target}\n{e.output.decode()}")

def main():
    parser = argparse.ArgumentParser(description="Ping a device. WARNING: This tool is vulnerable to command injection.")
    parser.add_argument("target", type=str, help="IP address or DNS name of the target device")
    args = parser.parse_args()
    ping_device(args.target)

if __name__ == "__main__":
    main()
