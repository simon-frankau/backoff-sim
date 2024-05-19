import matplotlib
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np

matplotlib.use('agg')

data = np.genfromtxt('compare.csv', delimiter = ',', names = True)

fig, ax = plt.subplots()
# ax.plot(data['Timeslot'], [data['BEB'], data['MBEB']], labels=['Binary Exponential Backoff', 'Modified Binary Exponential Backoff'])
ax.plot('Timeslot', 'BEB', data=data, label = 'Binary Exponential Backoff')
ax.plot('Timeslot', 'MBEB', data=data, label = 'Modified Binary Exponential Backoff')
ax.legend(loc='upper right')
ax.set_title('Retry attempts under BEB vs. MBEB')
ax.set_xlabel('Retry slot')
ax.set_ylabel('Fraction of clients retrying')

# plt.show()
plt.savefig('backoff-compare.png')
