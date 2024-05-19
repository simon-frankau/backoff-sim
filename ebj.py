import matplotlib
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np

matplotlib.use('agg')

data = np.genfromtxt('ebj.csv', delimiter = ',', names = True)

fig, ax = plt.subplots()
ax.stackplot(data['Time'], [data['Retry_1'], data['Retry_2'], data['Retry_3'], data['Retry_4'], data['Retry_5']], labels=['Retry 1', 'Retry 2', 'Retry 3', 'Retry 4', 'Retry 5'], alpha=0.8)
ax.legend(loc='upper right')
ax.set_title('Retry rate with exponential back-off with jitter')
ax.set_xlabel('Time')
ax.set_ylabel('Retry rate')

# plt.show()
plt.savefig('backoff-ebj.png')
