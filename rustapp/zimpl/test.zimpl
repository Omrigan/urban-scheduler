# set E_idx := { read "events_idx.dat" as "<1n>" comment "#" };
# param E := read "events.dat" as <1s> n+;
# param dist :=  read "dists.dat" as "<1n, 2n> 3n";

set E_idx    := {1, 2};
set E[E_idx] := <1> {1, 2}, <2> {3, 4};

set P        := union <e_idx> in E_idx: E[e_idx];
set PP       := P*P;

param dist[PP]     := <1, 2> 1 default 100;
param P_from[P]    := <1> 1, <2> 1;

set EE_idx := { <i, j> in E_idx*E_idx with i < j };

# Vars
var x [EE_idx] binary;
var u [EE_idx];
var y [P] binary;
var a [PP] binary;

minimize cost: sum <p1, p2> in PP: a[p1, p2] * dist[p1, p2];


subto aux: forall <i, j> in EE_idx do
            forall <p1> in E[i] do forall <p2> in  E[j] do
                a[p1, p2] == x[i,j] * y[p1] * y[p2];

subto from: forall <i> in E_idx - {1} do
   (sum <i, j> in EE_idx: x[i, j]) +
   (sum <j, i> in EE_idx: x[j, i]) == 2;

subto event_happens: forall <i> in E_idx do
    (sum <p> in E[i]: y[p]) == 1;