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
gtsp1 = load_problem('problems/gtsp1.yaml')


def request(endpoint, data):
    return post(URL + endpoint, json=data).json()


def compare(problem, alg1, alg2, normalize_order=False):
    problem = deepcopy(problem)
    schedules = []
    for algo in [alg1, alg2]:
        problem["config"]["solve_algorithm"] = algo
        result = request("/predict", problem)
        schedules.append(result.get("schedule"))
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


def measure(problem):
    solution = request("/predict", problem)
    stages = solution['report']['stages']

    for s in stages:
        if s['name'] == FROM_STAGE:
            from_time = s['timestamp']
        if s['name'] == TO_STAGE:
            to_time = s['timestamp']

    return to_time - from_time


def generate_tsp(base_problem, cnt):
    result = deepcopy(base_problem)
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
    result['events'][0]['items'] = events
    return result


def generate_ordered(base_problem, num_events, points_per_event):
    result = deepcopy(base_problem)
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
    result['events'] = events
    return result


def generate_gtsp(base_problem, num_events, points_per_event):
    result = deepcopy(base_problem)
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
    result['events'][0]['items'] = events

    return result


def tsp_generator(base_problem):
    def gen(cnt):
        return generate_tsp(base_problem, cnt)

    return gen


def points_generator(base_problem):
    def gen(x):
        return generate_ordered(base_problem, x, 3)

    return gen


def gtsp_generator(base_problem):
    def gen(x):
        return generate_gtsp(base_problem, x, 2)

    return gen


def gtsp_pts_per_event_generator(base_problem):
    def gen(x):
        return generate_gtsp(base_problem, 5, x)

    return gen


def make_plot(name, it, base_func):
    f = open('results/' + name, 'w')
    for elem in it:
        result = []
        for i in range(SIZE):
            problem = base_func(elem)
            delta = measure(problem)
            result.append(delta)
        line = "%s %s %s" % (elem, np.mean(result), np.std(result))
        print(line)
        f.write(line + '\n')
    f.close()


def ensure_correct():
    compare(practical1, "generic", "opt")
    compare(gtsp1, "generic", "opt", normalize_order=True)
    compare(tsp1, "generic", "opt", normalize_order=True)
    tsp2 = generate_tsp(tsp1, 7)
    compare(tsp2, "generic", "opt", normalize_order=True)
    gtsp2 = generate_gtsp(gtsp1, 5, 3)
    compare(gtsp2, "generic", "opt", normalize_order=True)


def plot_gtsp_pts_per_event():
    print("Generic")
    make_plot('gtsp_pts_per_event_generic.dat', range(1, 20, 2), gtsp_pts_per_event_generator(tsp1))
    tsp1_opt = deepcopy(tsp1)
    tsp1_opt["config"]["solve_algorithm"] = 'opt'
    print("Opt")
    make_plot('gtsp_pts_per_event_opt.dat', range(1, 20, 2), gtsp_pts_per_event_generator(tsp1_opt))


def plot_tsp():
    print("Generic")
    make_plot('tsp_generic.dat', range(3, 9), tsp_generator(tsp1))
    tsp1_opt = deepcopy(tsp1)
    tsp1_opt["config"]["solve_algorithm"] = 'opt'
    print("Opt")
    make_plot('tsp_opt.dat', range(3, 20), tsp_generator(tsp1_opt))


if __name__ == "__main__":
    # pretty_print(generate_gtsp(tsp1, 6, 2))
    ensure_correct()
    # tsp2 = generate_tsp(tsp1, 8)
    # compare(tsp2, "generic", "opt", normalize_order=True)
    # plot_gtsp_pts_per_event()
    plot_tsp()
