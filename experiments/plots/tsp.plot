set terminal pdf
set output "plot_results/tsp.pdf"

set logscale y 2
set xlabel 'Number of events'
set ylabel 'Time, ms'
set title 'Time for tsp'

plot "results/tsp_generic.dat" with errorbars pointtype 7 pointsize 0.7 t 'partial-order', \
        "results/tsp_opt.dat" with errorbars pointtype 7 pointsize 0.7 t 'SCIP'

