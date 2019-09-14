# set E_idx := {{ read "events_idx.dat" as "<1n>" comment "#" }};
# param E := read "events.dat" as <1s> n+;
# param dist :=  read "dists.dat" as "<1n, 2n> 3n";

set E_idx           := {E_idx};
set E_prev[E_idx]   := {E_prev};
set E[E_idx]        := {E};

set P        := union <e_idx> in E_idx: E[e_idx];
set PP       := P*P;

param dist[PP]     := read "/tmp/{id}/dists.dat" as "<1n, 2n> 3n" default 100;
param N            := max <i> in E_idx : i + 1;

set EE_idx := {{ <i, j> in E_idx*E_idx with i != j }};

# Vars
var x [EE_idx] binary;
var u [E_idx] real >= 0;
var y [P] binary;
var a [PP] binary;
var b [PP] binary;

minimize cost: sum <p1, p2> in PP: a[p1, p2] * dist[p1, p2];


subto aux: forall <i, j> in EE_idx do
            forall <p1> in E[i] do forall <p2> in  E[j] do
                a[p1, p2] == x[i,j] * b[p1, p2] and
                b[p1, p2] == y[p1] * y[p2];

subto from: forall <i> in E_idx do
   (sum <i, j> in EE_idx: x[i, j]) == 1;

subto toto: forall <j> in E_idx do
   (sum <i, j> in EE_idx: x[i, j]) == 1;

subto event_happens: forall <i> in E_idx do
    (sum <p> in E[i]: y[p]) == 1;

subto subtour: forall <i, j> in EE_idx do
    if i + 1 < N and j + 1 < N then
        u[i] - u[j] + N*x[i, j] <= N - 1 end;


subto order: forall <i> in E_idx do
                forall <j> in E_prev[i] do
                    u[i] >= u[j];