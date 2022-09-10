import matplotlib.pyplot as plt
import numpy as np
import json


with open('./out.txt', 'r') as f:
    data = json.loads(f.read())

plt.scatter(
    [x[0] for x in data],
    [x[1] for x in data],
)
plt.show()
