#! /usr/bin/env python
# -*- coding: utf-8 -*-

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.gridspec import GridSpec

def format_axes(fig):
    for i, ax in enumerate(fig.axes):
        ax.text(0.5, 0.5, "ax%d" % (i+1), va="center", ha="center")
        ax.tick_params(labelbottom=False, labelleft=False)

fig = plt.figure(constrained_layout=True)

gs = GridSpec(3, 3, figure=fig)
ax1 = fig.add_subplot(gs[0, 0])
ax2 = fig.add_subplot(gs[0, 1])
ax3 = fig.add_subplot(gs[0, 2])
ax4 = fig.add_subplot(gs[1:3,:])

fig.suptitle("Piecewise Function Example")

x1 = np.linspace(-1,1,101)
y1 = np.polyval([2,0,-2],x1)
ax1.plot(x1,y1)
ax1.text(0,-0.5, "fcn1", va="center", ha="center")
ax1.grid(True)
ax1.set_xlim([-1,1])
ax1.set_ylim([-2,0])


x2 = np.linspace(0,1,101)
y2 = np.polyval([4,0],x2)
ax2.plot(x2,y2)
ax2.text(0.5,0.5, "fcn2", va="center", ha="center")
ax2.grid(True)
ax2.set_xlim([0,1])
ax2.set_ylim([0,4])

x3 = np.linspace(0,1.5,101)
y3 = np.polyval([-4,0,4],x3)
ax3.plot(x3,y3)
ax3.text(0.5,0.5, "fcn3", va="center", ha="center")
ax3.grid(True)
ax3.set_xlim([0,1.5])
ax3.set_ylim([-5,4])

x = np.linspace(0.0, 0.5, 101)
x = np.concatenate( (x,np.linspace(0.0,1.0,101)+0.5) )
x = np.concatenate( (x,np.linspace(0.0,3.0,101)+0.5+1.0) )
y = np.concatenate( (y1,y2,y3) )

ax4.plot( x,y )
ax4.grid(True)
ax4.set_xlim([0,4.5])
ax4.set_ylim([-5,4])

plt.show()
