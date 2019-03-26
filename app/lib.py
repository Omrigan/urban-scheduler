import json
import yaml

cities = yaml.load(open('cities.yaml'))


def pretty_json(inp):
    return json.dumps(inp, indent=4, ensure_ascii=False)
