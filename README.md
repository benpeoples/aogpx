# AOGPX

This is a very simple tool that takes Atlas Obscura's database of points of interest and generates a CSV list of POIs that are within 10 miles of the GPX track.

# AO Database

There is a file in this repo (and the distributions) called `ao_db.json` -- this is a JSON document that consists of a single array of objects in the following format:

```json
{"id":51949,"lat":18.94487,"lng":72.833672}
```

`id` is the AO ID of the POI, lat/lng are the respective latitude and longitude of the POI.

# Optimizations

We set an initial bounding box that is 0.5 degrees larger than the max/min points of the track, this allows us to rapidly exclude points that couldn't possibly be within range.

It is currently hard-coded to a 10.0 mile distance, but this could be later modified to be a commandline argument.

# Use

```
ao_gpx -a ao_db.json -g ragbrai.gpx -v
```

Quite simply, `-a` tells us what AO database to use, and `-g` tells us what GPX file to load.  It will only load the first track in the GPX file.   `-v` enables verbose mode, which will print some additional info above the CSV file.   Otherwise it simply prints a CSV header and then one line per entry:

```
"Distance","Title","City","URL"
1.5491,"'Sitting Man'","Iowa City, Iowa",https://www.atlasobscura.com/places/sitting-man-iowa-city
0.1214,"'Better Homes and Gardens' Test Garden","Des Moines, Iowa",https://www.atlasobscura.com/places/better-homes-and-gardens-test-garden
0.1464,"West End Architectural Salvage","Des Moines, Iowa",https://www.atlasobscura.com/places/west-end-architectural-salvage
0.1580,"Zombie Burger + Drink Lab","Des Moines, Iowa",https://www.atlasobscura.com/places/zombie-burger
2.7884,"High Trestle Trail Bridge","Madrid, Iowa",https://www.atlasobscura.com/places/high-trestle-trail-bridge
```

10 miles is a pretty good distance for rural areas, but it really tends to capture an excessive number of POIs in urban areas.
