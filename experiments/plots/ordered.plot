set terminal pdf
set output "plot_results/ordered.pdf"

set logscale y 2
set xlabel 'Number of points per event'
set ylabel 'Time, ms'
set title 'Time for ordered'

plot "results/ordered_generic.dat" with errorbars pointtype 7 pointsize 0.7 t 'partial-order', \
        "results/ordered_opt.dat" with errorbars pointtype 7 pointsize 0.7 t 'SCIP', \
         "results/ordered_ordered.dat" with errorbars pointtype 7 pointsize 0.7 t 'full-order'


