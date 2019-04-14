import yaml
from subprocess import call

cities = yaml.load(open('../app/cities.yaml'))

read_str = " ".join("--read-pbf ../data/osm-files/%s.osm.pbf" % city for city in cities)

merge_str = "--merge " * (len(cities) - 1)

final_cmd = "osmosis %s %s --write-pbf ../data/osm-files/osrm-final.osm.pbf" % (read_str, merge_str)

print(final_cmd)

# call(final_cmd, shell=True)

call("cd ../deploy/ && ./preprocess-osrm.sh", shell=True)
