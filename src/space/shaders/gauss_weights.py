import sys
import math

num = int(sys.argv[1])

def curve(x, p):
    gauss = lambda x, f: 1.0 / math.exp((x*f)**2)

    return gauss(x * p, 0.2) + gauss(x * p, 0.6) + gauss(x * p, 1.0)


weights = []
weights_sum = 1.0

p = int(sys.argv[2])

# while weights[-1] < 0.01:
#     weights = []
#     for n in range(0, num):
#         x = n / (num - 1)
#
#         weights.append(curve(x, p))
#     weights_sum = sum(weights) + sum(weights[1:])
#     weights = [w / weights_sum for w in weights]
#     print(f"Sum    : {weights_sum} (P: {p})")
#     p = p * 0.9

for n in range(0, num):
    x = n / (num - 1)

    weights.append(curve(x, p))
weights_sum = sum(weights) + sum(weights[1:])
weights = [w / weights_sum for w in weights]

for w in weights:
    print(f"{w:f},")
