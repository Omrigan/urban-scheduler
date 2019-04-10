import yaml
from subprocess import call

cities = yaml.load(open('../app/cities.yaml'))

target_city = "helsinki"
city = cities[target_city]

FROM_NAME = "finland-latest.osm.pbf"

from_file = "../data/osm-files/%s" % FROM_NAME

city_file = "../data/osm-files/%s.osm.pbf" % target_city
nodes_file = "../data/osm-files/%s-nodes.osm" % target_city

bbox_str = " ".join("%s=%s" % pair for pair in city["bbox"].items())

#  --bounding-box top=49.5138 left=10.9351 bottom=49.3866 right=11.201 -

call("osmosis --read-pbf %s --bounding-box %s --write-pbf %s" % (
    from_file, bbox_str, city_file), shell=True)

cmd = "osmosis --read-pbf %s \
--tf reject-ways highway=motorway,motorway_link \
--tf reject-relations \
--tf accept-nodes \
--write-xml %s" % (city_file, nodes_file)

print(cmd)

call(cmd, shell = True)
