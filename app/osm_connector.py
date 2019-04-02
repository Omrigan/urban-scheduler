# BASE_URL = 'http://router.project-osrm.org/table/v1'

import numpy as np
import requests, os

BASE_URL = os.getenv('OSRM_URL')


def calcualte_distances(profile, src, dist):
    coords_list = []
    for lst in [src, dist]:
        for item in lst:
            lng = item['location']['lng']
            lat = item['location']['lat']
            coords_list.append("%s,%s" % (lng, lat))
    result_coords = ";".join(coords_list)
    sources = ";".join(str(x) for x in range(0, len(src)))
    destinations = ";".join(str(x) for x in range(len(src),
                                                  len(src) + len(dist)))
    url = "%s/%s/%s" % (BASE_URL, profile, result_coords)
    result = requests.get(url, params={
        "sources": sources,
        "destinations": destinations
    })
    result = result.json()
    arr = np.asarray(result.get('durations'), dtype='float64')
    arr[np.isnan(arr)] = np.infty
    arr = arr.astype('float64')
    print("Durations", arr.shape, arr.dtype, len(result.get('durations')[0]), list(result), flush=True)

    # print( / 60)

    return arr / 60


dist = [{'brand': 'парк "зарядье"', 'location': {'lat': 55.7515994, 'lng': 37.6288575}, 'name': 'Парк "Зарядье"'},
        {'brand': 'парк "горка"', 'location': {'lat': 55.7558691, 'lng': 37.6359123}, 'name': 'Парк "Горка"'},
        {'brand': 'park', 'location': {'lat': 55.7563636, 'lng': 37.6359929}, 'name': 'Park'},
        {'brand': 'парк трейд', 'location': {'lat': 55.75551220000001, 'lng': 37.6077401}, 'name': 'Парк Трейд'},
        {
            'brand': 'музейный парк', 'location': {'lat': 55.7589777, 'lng': 37.62769979999999},
            'name': 'Музейный парк'
        }] * 2

src = [
          {'brand': 'wine and crab', 'location': {'lat': 55.7584657, 'lng': 37.6237843}, 'name': 'Wine and Crab'},
          {
              'brand': 'высота 5642', 'location': {'lat': 55.75592330000001, 'lng': 37.62768570000001},
              'name': 'Высота 5642'
          },
          {
              'brand': 'белуга', 'location': {'lat': 55.7565202, 'lng': 37.6139581},
              'name': 'Белуга'
          },
          {
              'brand': 'порто мальтезе',
              'location': {'lat': 55.7527595, 'lng': 37.6268736}, 'name': 'Порто Мальтезе'
          },
          {
              'brand': 'bolshoi',
              'location': {
                  'lat': 55.76132759999999,
                  'lng': 37.6182096
              }, 'name': 'Bolshoi'
          }] * 2
# calcualte_distances("car", src, dist)
