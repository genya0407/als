set terminal dumb
set timefmt "%Y-%m-%dT%H:%M:%S+09:00"
set xdata time
plot '<cat' using 1:2 with line