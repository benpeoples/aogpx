extern crate gpx;
extern crate cheap_ruler;
#[macro_use] extern crate geo_types;

use clap::Parser;

use std::collections::HashSet;
use std::collections::HashMap;
use std::f32::INFINITY;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::fs;

use gpx::read;
use gpx::{Gpx, Track, TrackSegment};

use cheap_ruler::{CheapRuler, DistanceUnit};
use geo_types::Point;

#[derive(Serialize, Deserialize,Debug)]
struct Place {
    title: String,
    location: String,
    url: String
}

#[derive(Serialize, Deserialize,Debug)]
struct AObject {
    id: u32,
    lat: f64,
    lng: f64
}

#[derive(Debug)]
struct AList {
    id: u32,
    pnt: Point
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// AO Directory
   #[arg(short, long)]
   ao: String,

   /// GPX file
   #[arg(short, long)]
   gpx: String,

    /// Some extra debugging output
    #[arg(short, long, action)]
    verbose: bool,

}



fn main() {

    let args = Args::parse();

    let file = File::open(args.gpx).unwrap();
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = read(reader).unwrap();

    // Each GPX file has multiple "tracks", this takes the first one.
    let track: &Track = &gpx.tracks[0];
    if args.verbose {
        println!("GPX track name: {:?}",track.name);
    }
    
    let ruler = CheapRuler::new(40.5, DistanceUnit::Miles);

    let mut last_point = point!(x: 0.0,y : 0.0);
    let mut first_pass = true;

    let mut point_list : Vec<Point> = Vec::new();


    for segment in &track.segments {
        for point in &segment.points {
            if first_pass {
                last_point = point.point();
                first_pass = false;
                // println!("{},{}",last_point.x(),last_point.y());
                point_list.push(last_point);
            } else {
                let dist = ruler.distance(&last_point, &point.point());
                if dist >= 0.125 {
                    // println!("{},{}",point.point().x(),point.point().y());
                    last_point = point.point();
                    point_list.push(last_point);
                }
            }
        }

        if let Some(p) = &segment.points.last() {
            point_list.push(p.point());
        }

        if args.verbose {
        println!("Points in GPX Track: {:?}",point_list.len());
        }

        // at this point we have a point list.

        // let file = File::open("test.json").unwrap();
    }

    let mut max_lat : f64 = f64::NEG_INFINITY;
    let mut min_lat : f64 = f64::INFINITY;
    let mut max_lng : f64 = f64::NEG_INFINITY;
    let mut min_lng : f64 = f64::INFINITY;

    for points in &point_list {
        if points.x() > max_lat {
            max_lat = points.x();
        }
        if points.x() < min_lat {
            min_lat = points.x();
        }

        if points.y() > max_lng {
            max_lng = points.y();
        }
        if points.y() < min_lng {
            min_lng = points.y();
        }
    }

    max_lat += 0.5;
    min_lat -= 0.5;

    max_lng += 0.5;
    min_lng -= 0.5;

    if args.verbose {
    println!("Bounding rectangle lat: ({},{}) lng: ({},{})",min_lat,max_lat,min_lng,max_lng);
    }

    let ao = fs::read_to_string(args.ao).unwrap();

    let ao_list : Vec<AObject> = serde_json::from_str(&ao).unwrap();
   
    let mut ao_points : Vec<AList> = Vec::new();

    // println!("{:?}",ao_list);

    for item in ao_list {
        if item.lng < max_lat && item.lng > min_lat && item.lat < max_lng && item.lat > min_lng {
            let p = Point::new(item.lng, item.lat);
            let o : AList = AList { id: item.id, pnt: p };
            ao_points.push(o);
        }
    }

    if args.verbose {
    println!("Total pois in db: {:?}",ao_points.len());
    }

    let mut pois = HashSet::new();

    for trkpt in &point_list {
        for ao in &ao_points {
            let dist = ruler.distance(&trkpt, &ao.pnt);
            // println!("{:?} {:?} {}",trkpt,ao.pnt,dist);
            if dist <= 10.0 {
                pois.insert(ao.id);
            }
        }
    }

    println!("\"Distance\",\"Title\",\"City\",\"URL\"");

    for poi in &pois {
        let mut min_dist = f64::INFINITY;

        let mut thispt : Point = point!{x: 0.0, y: 0.0};

        for ao in &ao_points {
            if &ao.id == poi {
                thispt = ao.pnt;
                break;
            }
        }

        for trkpt in &point_list {
            let dist = ruler.distance(&thispt, trkpt);

            if dist < min_dist {
                min_dist = dist;
            }

        }

        // println!("Pt: {} Min_dist: {}",poi,min_dist);

        let resp = reqwest::blocking::get(format!("https://www.atlasobscura.com/places/{}.json?place_only=1",poi)).unwrap()
        .json::<Place>().unwrap();
        // println!("{:?}", resp);

        println!("{:.4},\"{}\",\"{}\",{}",min_dist,resp.title,resp.location,resp.url);

    }


}