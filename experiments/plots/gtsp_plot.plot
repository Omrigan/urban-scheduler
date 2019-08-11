set term png
set output "plots/gtsp.png"

set logscale y 2
plot "results/gtsp_generic.dat" with errorbars pointtype 7 pointsize 2 t 'My', "results/gtsp_opt.dat" with errorbars pointtype 7 pointsize 2 t 'SCIP'

set xlabel 'Buffer Size, bytes'
set ylabel 'Time, seconds'
set title 'Influence of Buffer Size'