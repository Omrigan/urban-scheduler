set terminal pdf
set output "plot_results/gtsp_pts_per_event.pdf"

set logscale y 2
set xlabel 'Points per event'
set ylabel 'Time, ms'
set title 'Time for gtsp'

plot "results/gtsp_pts_per_event_generic.dat" with errorbars pointtype 7 pointsize 0.7 t 'partial-order', \
        "results/gtsp_pts_per_event_opt.dat" with errorbars pointtype 7 pointsize 0.7 t 'SCIP'

