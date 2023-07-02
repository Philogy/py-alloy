import alloy
import os


def main():
    # Generate random byte strings
    b1 = os.urandom(32)
    b2 = os.urandom(32)

    # ABI encode bytes32[] array `[b1, b2]`
    encoded = (0x20).to_bytes(32, 'big') + (2).to_bytes(32, 'big') + b1 + b2

    arr = alloy.decode('bytes32[]', encoded)

    print(f'arr: {arr}')

    assert len(arr) == 2

    out1, out2 = arr

    assert out1 == b1
    assert out2 == b2


if __name__ == '__main__':
    main()
