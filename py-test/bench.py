import eth_abi_lite as eth_abi
import py_alloy as alloy
import json
import pyperclip
from math import floor
import time
from string import printable
from random import random, randint, choice, seed


def random_type():
    # base_type = choice(['int', 'uint', 'bytesN', 'bytes', 'string', 'address'])
    base_type = choice(['uint', 'bytesN', 'bytes', 'string', 'address'])
    match base_type:
        case 'int' | 'uint':
            return f'{base_type}{randint(1, 32) * 8}'
        case 'bytesN':
            return f'bytes{randint(1, 32)}'
        case 'bytes' | 'string' | 'address':
            return base_type
        case _:
            raise ValueError(f'Unrecognized base type {base_type!r}')


LENGTH_CAP = 8


def rand_len(n, cap=LENGTH_CAP):
    return min(floor(n / random()), cap)


def rand_bytes(length):
    return bytes([

        randint(0, 0xff)
        for _ in range(length)
    ])


def random_value(t: str):
    match t:
        case 'bytes':
            return rand_bytes(rand_len(16, cap=128))
        case 'address':
            return f'0x{rand_bytes(20).hex()}'
        case 'string':
            return ''.join([
                choice(printable)
                for _ in range(rand_len(16, cap=128))
            ])
        case _ if t.startswith('int'):
            bits = int(t[3:])
            return randint(0, (1 << bits) - 1) - (1 << (bits - 1))
        case _ if t.startswith('uint'):
            bits = int(t[4:])
            return randint(0, (1 << bits) - 1)
        case _ if t.startswith('bytes'):
            length = int(t[5:])
            return rand_bytes(length)
        case _:
            raise TypeError(f'Unrecognized type {t!r}')


def random_next():
    t = random_type()
    return t, lambda: random_value(t)


def arrayify(f, n):
    return lambda: [f() for _ in range(n)]


def rand_arrayif(fixed, t, v):
    length = rand_len(2)
    return (
        f'{t}[{length if fixed else ""}]',
        arrayify(v, length)
    )


def rand_arr(t, v, p=0.2):
    r = random()
    if r < p:
        return rand_arrayif(r < p/2, t, v)
    return t, v


def tupleify_type(t):
    return '(' + ','.join(t) + ')'


def random_expand(types, values):
    r = random()

    if r < 0.9:
        new_t, new_v = rand_arr(*random_next())
        return types + [new_t], values + [new_v]
    elif len(types) >= 1:
        slice_back = randint(1, len(types))
        if slice_back == 1:
            new_t, new_v = rand_arrayif(
                r < 0.95,
                types[-slice_back],
                values[-slice_back]
            )
        else:
            t_slice = types[-slice_back:]
            v_slice = values[-slice_back:]
            # Join types to tuple type
            new_t, new_v = rand_arrayif(
                r < 0.95,
                tupleify_type(t_slice),
                lambda: [v() for v in v_slice]
            )
        return (
            types[:-slice_back] + [new_t],
            values[:-slice_back] + [new_v]
        )

    return types, values


def random_abi_obj(base_size_factor):
    t = []
    v = []

    length = rand_len(base_size_factor)
    for _ in range(length):
        t, v = random_expand(t, v)

    return t, [g() for g in v]


def random_simple_obj(base_size_factor):
    t = []
    v = []

    length = rand_len(base_size_factor)
    for _ in range(length):
        new_t, new_v = rand_arr(*random_next())
        t.append(new_t)
        v.append(new_v)

    return t, [g() for g in v]


def to_alloy_type(t):
    if len(t) == 1:
        return t[0]
    return tupleify_type(t)


def show_bytes(b: bytes):
    for w in range((len(b) + 31) // 32):
        print(b[w * 32: (w+1) * 32].hex())


def benchmarked(f):
    def _inner_benchmarked(*args, **kwargs):
        start = time.time()
        res = f(*args, **kwargs)
        return time.time() - start, res
    return _inner_benchmarked


@benchmarked
def run_eth_abi(examples):
    for t, _, encoded, _ in examples:
        eth_abi.decode_abi(t, encoded)


@benchmarked
def run_alloy(examples):
    for _, at, encoded, _ in examples:
        alloy.decode(at, encoded)


def do_bench(obj_generator, total_examples=10_000):
    print(f'Generating {total_examples} examples')
    start = time.time()
    examples = []
    for i in range(total_examples):
        # Replace `random_simple_obj` with `random_abi_obj` for intricate data benchmark
        t, v = obj_generator()
        at = to_alloy_type(t)
        encoded = eth_abi.encode_abi(t, v)
        examples.append((t, at, encoded, v))

    print(f'Generated in {time.time() - start:.2f}s')

    alloy_time, _ = run_alloy(examples)
    eth_abi_time, _ = run_eth_abi(examples)

    print(f'eth_abi_time: {eth_abi_time:.4f}')
    print(
        f'alloy_time: {alloy_time:.4f} ({eth_abi_time / alloy_time - 1:.2%})'
    )


def main():
    rand_seed = randint(0, 1_000_000)
    # rand_seed = 578093
    print(f'rand_seed: {rand_seed}')
    seed(rand_seed)

    print('\n===== Simple ABI Obj Decode Bench ===========')
    do_bench(lambda: random_simple_obj(3))

    print('\n===== Intricate ABI Obj Decode Bench ===========')
    do_bench(lambda: random_abi_obj(3))


if __name__ == '__main__':
    main()
