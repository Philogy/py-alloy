import alloy
import os


def main():
    # Input values
    b1 = os.urandom(32)
    b2 = os.urandom(32)
    x = int.from_bytes(os.urandom(32), 'big')

    # Test sequential decode
    encoded = (0x40).to_bytes(32, 'big')\
        + (x).to_bytes(32, 'big')\
        + (2).to_bytes(32, 'big')\
        + b1\
        + b2
    byte_arr, x_out = alloy.decode('(bytes32[],uint256)', encoded)
    assert x_out == x
    assert byte_arr == [b1, b2]

    # Test single decode
    encoded = x.to_bytes(32, 'big')
    x_out = alloy.decode('uint256', encoded)
    assert x_out == x


if __name__ == '__main__':
    main()
