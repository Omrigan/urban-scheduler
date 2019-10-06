from requests import get, post
from yaml import load
import numpy as np
import json
from copy import deepcopy

# URL = "https://api.rc.urbanscheduler.ml"
URL = "http://localhost:89"

SIZE = 3

FROM_STAGE = 'preprocessed'
TO_STAGE = 'solved'


def pretty_print(inp):
    print(json.dumps(inp, indent=4, ensure_ascii=False))


def load_problem(name):
    f = open(name)
    content = load(f)
    return content


tsp1 = load_problem('problems/tsp1.yaml')
practical1 = load_problem('problems/practical1.yaml')
practical2 = load_problem('problems/practical2.json')
gtsp1 = load_problem('problems/gtsp1.yaml')
ordered1 = load_problem('problems/ordered1.yaml')


def request(endpoint, data):
    return post(URL + endpoint, json=data).json()


def predict(problem):
    solution = request("/predict", problem)
    if 'report' not in solution:
        print(solution)
    stages = solution['report']['stages']

    for s in stages:
        if s['name'] == FROM_STAGE:
            from_time = s['timestamp']
        if s['name'] == TO_STAGE:
            to_time = s['timestamp']

    return solution.get("schedule"), (to_time - from_time)


def measure_time(problem):
    all_times = []
    for i in range(SIZE):
        result, delta = predict(problem)

        all_times.append(delta)
    return (np.mean(all_times), np.std(all_times))


def single_compare(problem, alg1, alg2, normalize_order=False):
    problem = deepcopy(problem)
    schedules = []
    time_results = dict()
    for algo in [alg1, alg2]:
        problem["config"]["solve_algorithm"] = algo
        result, delta = predict(problem)
        time_results[algo] = delta
        schedules.append(result)
    assert len(schedules[0]) == len(schedules[1])
    if normalize_order:
        # for shift in range(len(schedules[0])):
        #     if schedules[0][shift] == schedules[1][0]:
        #         break
        # schedules[0] = schedules[0][shift:] + schedules[0][:shift]
        if schedules[0][0]['idx'] == schedules[1][-1]['idx']:
            schedules[0] = list(reversed(schedules[0]))
    for p1, p2 in zip(*schedules):
        del p1['color']
        del p2['color']
        print(p1, p2)
    print()
    for p1, p2 in zip(*schedules):
        assert p1 == p2
    print("%s and %s are correct" % (alg1, alg2))
    return time_results


def wrap_events(events, algo="generic", clipping=10, solver="rust", version=2):
    if version == 2:
        return {"config": {
            "dists_method": "dummy",
            "clipping": clipping,
            "solver": solver,
            "solve_algorithm": algo,
            "city": "moscow",
            "final_route": False
        }, "version": 2, "events": events}
    else:
        return {"config": {
            "dists_method": "dummy",
            "clipping": clipping,
            "solver": solver,
            "solve_algorithm": algo,
            "city": "moscow",
            "final_route": False
        }, "version": 1, "ordered_events": events}


def generate_tsp(cnt):
    events = []
    for i in range(cnt):
        event = {
            "type": "fixed_place",
            "location": {
                "lat": np.random.uniform(0, 100),
                "lng": np.random.uniform(0, 100)
            }
        }
        events.append(event)
    return [{"type": "parallel", "name": "parallel", "items": events}]


def generate_ordered(num_events, points_per_event):
    events = []
    for i in range(num_events):
        points = []
        for j in range(points_per_event):
            points.append({
                "coords": (np.random.uniform(0, 100), np.random.uniform(0, 100)),
                "idx": 0
            })

        events.append({
            "type": "points",
            "points": points
        })
    return events


def generate_gtsp(num_events, points_per_event):
    events = []
    for i in range(num_events):
        points = []
        for j in range(points_per_event):
            points.append({
                "coords": (np.random.uniform(0, 100), np.random.uniform(0, 100)),
                "idx": 0
            })

        events.append({
            "type": "points",
            "points": points
        })
    return [{"type": "parallel", "name": "parallel", "items": events}]


def make_plot(name, it, base_func):
    f = open('results/' + name, 'w')
    for elem in it:
        mean, std = measure_time(base_func(elem))
        line = "%s %s %s" % (elem, mean, std)
        print(line)
        f.write(line + '\n')
    f.close()


def ensure_correct():
    # compare(practical1, "generic", "opt")
    single_compare(ordered1, "generic", "opt")
    single_compare(ordered1, "generic", "ordered")
    single_compare(practical2, "generic", "opt")
    single_compare(gtsp1, "generic", "opt", normalize_order=True)
    single_compare(tsp1, "generic", "opt", normalize_order=True)
    tsp2 = wrap_events(generate_tsp(7), "generic")
    single_compare(tsp2, "generic", "opt", normalize_order=True)
    gtsp2 = wrap_events(generate_gtsp(5, 3), "opt")
    single_compare(gtsp2, "generic", "opt", normalize_order=True)


def plot_gtsp():
    print("Generic")

    def generic_func(x):
        return wrap_events(generate_gtsp(4, x), algo="generic")

    make_plot('gtsp_generic.dat', range(1, 20, 1), generic_func)

    print("Opt")

    def opt_func(x):
        return wrap_events(generate_gtsp(4, x), algo="opt")

    make_plot('gtsp_opt.dat', range(1, 11, 1), opt_func)


def plot_tsp():
    print("Generic")

    def generic_func(x):
        return wrap_events(generate_tsp(x), algo="generic")

    make_plot('tsp_generic.dat', range(3, 11), generic_func)

    print("Opt")

    def opt_func(x):
        return wrap_events(generate_tsp(x), algo="opt")

    make_plot('tsp_opt.dat', range(3, 15), opt_func)


def plot_ordered():
    # print("Python ordered")
    #
    # def opt_func(x):
    #     return wrap_events(generate_ordered(4, x), algo="ordered", solver="python", version=1)
    #
    # make_plot('ordered_ordered.dat', range(1, 100), opt_func)

    print("Generic")

    def generic_func(x):
        return wrap_events(generate_ordered(4, x), algo="generic")

    make_plot('ordered_generic.dat', range(1, 100), generic_func)

    print("Opt")

    def opt_func(x):
        return wrap_events(generate_ordered(4, x), algo="opt")

    make_plot('ordered_opt.dat', range(1, 10), opt_func)

    print("Ordered")

    def opt_func(x):
        return wrap_events(generate_ordered(4, x), algo="ordered")

    make_plot('ordered_ordered.dat', range(1, 100), opt_func)


def single_measurement(name, algo, problem):
    mean, var = measure_time(problem)
    print("%s & %s & %s & %s \\\\" % (name, algo, mean, var))


def make_table():
    print("""\\begin{tabular}{|c c || c c|}
\\hline
Problem & Algorithm & Time mean, ms & Time variance \\\\
\\hline \\hline""")

    single_measurement("Ordered", "full-order", wrap_events(ordered1['events'], algo="ordered", clipping=40))
    single_measurement("Ordered", "partial-order", wrap_events(ordered1['events'], algo="generic", clipping=40))
    single_measurement("Ordered", "SCIP", wrap_events(ordered1['events'], algo="opt", clipping=20))
    single_measurement("Practical", "partial-order", wrap_events(practical2['events'], algo="generic"))
    single_measurement("Practical", "SCIP", wrap_events(practical2['events'], algo="opt"))

    print("""\\hline
\end{tabular}""")

if __name__ == "__main__":
    # ensure_correct()
    # plot_gtsp()
    # plot_tsp()
    # plot_ordered()
    make_table()
