# Übungsblatt 07 Aufgabe 4) Berechnen von Pollards-rho-Algorithmus
# Autoren des Skripts: Pascal Botzum u. J. Leander Zellentin
# 
# Usage: python Uebung_07_4_Botzum_Zellentin {N} {x_0}
#         um N zu faktorisieren mit Startzahl x_0

import sys

def ggT(x: int, y: int) -> int:
    if y > x:
        return ggT(y, x)

    #print(f"ggT({x},{y}) mit Hilfe von Euklid\n")

    # ls = f * rs + r
    left_side = []
    factor = []
    right_side = []
    rest = []

    left_side.append(x)
    factor.append(int(x / y))
    right_side.append(y)
    rest.append(x % y)

    i = 0
    while rest[i] != 0:
        i += 1
        left_side.append( right_side[i - 1])
        right_side.append(rest[i - 1])
        factor.append(int(left_side[i] / right_side[i]))
        rest.append(left_side[i] % right_side[i])

    print(f'### ggT({x}, {y}) ###')
    #for j in range(i+1):
    #    print(f"{left_side[j]} = {factor[j]} * {right_side[j]} + {rest[j]}")
    print(f'### -> {right_side[i]} ###')
    
    return right_side[i]

# --- Initialisiere wichtige Variablen ---

N = sys.argv[1]; N = int(N)
x_0 = sys.argv[2]; x_0 = int(x_0)

f = lambda x: ((x + 1) ** 2) % N

x = [x_0]
y = [x_0]

g = 1
i = 0

# --- Haupt-Schleife ---------------------

while g == 1:
    x.append( f(x[i]) )
    y.append( f(f(y[i])) )
    i += 1
    g = ggT((y[i] - x[i]) % N, N)
    
match (g):
    case g if 1 < g < N: ...
    case _: g = 1
    
print(g)
