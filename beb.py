import matplotlib
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np

matplotlib.use('agg')

data = np.genfromtxt('beb.csv', delimiter = ',', names = True)

fig, ax = plt.subplots()
ax.stackplot(data['Timeslot'], [data['Retry_1'], data['Retry_2'], data['Retry_3'], data['Retry_4'], data['Retry_5']], labels=['Retry 1', 'Retry 2', 'Retry 3', 'Retry 4', 'Retry 5'], alpha=0.8)
ax.legend(loc='upper right')
ax.set_title('Retry attempts under Binary Exponential Back-off')
ax.set_xlabel('Retry slot')
ax.set_ylabel('Fraction of clients retrying')

# plt.show()
plt.savefig('backoff-beb.png')
