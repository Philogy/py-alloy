import py_alloy as alloy
import eth_abi_lite as eth_abi


def decode_erc20(encoded: bytes):
    selector = encoded[:4]


def main():
    res = alloy.ERC20.decode(bytes.fromhex(
        '0x18160ddd'[2:]
    ))
    print(f'res: {res}')
    print(f'dir(res): {dir(res)}')
    # print(f'res.to: {res.to}')
    # print(f'res.amount: {res.amount}')

    t = alloy.ERC20.TransferFromCall('a', 'b', 34)
    print(f't: {t}')


if __name__ == '__main__':
    main()
