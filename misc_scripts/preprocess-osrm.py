import yaml
from subprocess import call

cities = yaml.load(open('../app/cities.yaml'))

read_str = " ".join("--read-pbf ../data/osm-files/%s" % city for city in cities)

merge_str = "--merge " * (len(cities) - 1)

final_cmd = "osmosis %s %s --write-pbf ../data/osm-files/osrm-final.pbf" % (read_str, merge_str)
