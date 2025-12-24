import hid
import time

VENDOR_ID  = 0x04D9   # <-- CHANGE THIS
PRODUCT_ID = 0xA1CD   # <-- CHANGE THIS

def hex_bytes(s: str) -> bytes:
    return bytes.fromhex(s.replace(" ", ""))


HANDSHAKE = hex_bytes(
    "30 00 00 00 00 55 aa 00"
)

COMMIT_1 = hex_bytes(
    "08 00 3f 01 00 06 c4 3b"
)

COMMIT_2 = hex_bytes(
    "08 00 3f 01 00 04 c4 3b"
)

COMMIT_3 = hex_bytes(
    "08 00 3f 01 00 02 c4 3b"
)

RGB_WHITE = hex_bytes(
    "3f00003f3f00003f0000003f003f3f3f003f3f3f3f000000"
    "104a7a"
    "060000000000000000d80c820298bd6400a443c667000000"
    "005a000000f8be6400e08e4b05"
)

RGB_CYAN = hex_bytes(
    "3f00003f3f00003f0000003f003f3f3f003f3f3f3f000000"
    "00497a"
    "060000000000000000d80c820298bd6400a443c667000000"
    "005a000000f8be640000c34905"
)

RGB_GREEN = hex_bytes(
    "3f00003f3f00003f0000003f003f3f3f003f3f3f3f000000"
    "504e7a"
    "060000000000000000d80c820298bd6400a443c667000000"
    "005a000000f8be640060457a08"
)


def read_response(dev, label):
    try:
        data = dev.read(64, timeout_ms=500)
        if data:
            print(f"{label} IN:", bytes(data).hex())
    except Exception as e:
        print("Read error:", e)


def send_cycle(dev, rgb_payload, commit_payload, name):
    print(f"\n=== {name} ===")

    dev.send_feature_report(HANDSHAKE)
    time.sleep(0.02)

    dev.write(rgb_payload)
    read_response(dev, "ACK1")

    dev.send_feature_report(commit_payload)
    read_response(dev, "ACK2")

    time.sleep(0.5)


def main():
    dev = hid.Device(vid=VENDOR_ID, pid=PRODUCT_ID)

    dev.nonblocking = False

    send_cycle(dev, RGB_WHITE, COMMIT_1, "WHITE")
    send_cycle(dev, RGB_CYAN,  COMMIT_2, "CYAN")
    send_cycle(dev, RGB_GREEN, COMMIT_3, "GREEN")

    dev.close()


if __name__ == "__main__":
    main()
