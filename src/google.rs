use rand::Rng;
use google_maps::prelude::*;
use std::collections::HashMap;

pub fn addr_to_route(addrs_string: &String) -> String
{
    if addrs_string.is_empty()
    {
        return "Error: URL is Empty".to_owned()
    }
    
    let temp_string = addrs_string.to_string().replace("https://www.google.com/maps/dir/", "").replace("+", " ").replace("%E2%80%99", "'");
    let temp_vec: Vec<&str> = temp_string.split("/@").collect();
    let mut addr_vec: Vec<&str> = temp_vec[0].split("/").collect();

    if addr_vec.len() <= 2
    {
        return "Error: Not Enough Destinations Supplied".to_owned();
    }
    let mut waypoint_vec: Vec<Waypoint> = Vec::new();
    for addr in addr_vec.clone()
    {
        waypoint_vec.push(Waypoint::Address(String::from(addr)));
    }


    let distance_matrix = distance_matrix_api(waypoint_vec);
    let mut duraction_vec: Vec<i64> = Vec::new();

    for row in distance_matrix.rows
    {
        for element in row.elements
        {
            #[allow(for_loops_over_fallibles)]
            for duration in element.duration
            {
                duraction_vec.push(duration.value.num_minutes());
            }
        }
    }
    
    
    let mut travel_time: HashMap<PathHash, i64> = HashMap::new();
    let mut x = 0;

    
    if !((addr_vec.len() * addr_vec.len()) == duraction_vec.len())
    {
        return "Error: Address Total incorrect, please review the entered address's".to_string();
    }
    for origin in addr_vec.clone()
    {
        for destination in addr_vec.clone()
        {
            if !(duraction_vec[x] == 0)
            {
                travel_time.insert(PathHash::new(origin, destination), duraction_vec[x]);
                x += 1;
            }else {
                x += 1;
            }
        }
    }

    addr_vec.push(addr_vec[0]);

    let mut current_rt = addr_vec.clone();
    let mut current_dur = calculate_route_duration(addr_vec.clone(), &travel_time);

    let mut best_rt = addr_vec.clone();
    let mut best_dur = current_dur.clone();

    for _x in 0..50000
    {

        if current_dur < best_dur
        {
            best_rt.clear();
            best_rt = current_rt.clone();

            best_dur = current_dur;
        }

        current_rt = swap(current_rt.clone());
        current_dur = calculate_route_duration(current_rt.clone(), &travel_time);
    }

    let mut google_maps_link: String = "https://www.google.com/maps/dir/".to_string();
        for x in 0..best_rt.len()
        {
            google_maps_link.push_str(best_rt[x].replace(" ", "+").as_str());
            google_maps_link.push_str("/");
        }
        google_maps_link
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct PathHash
{
    origin: String,
    destination: String,
}
impl PathHash
{
    fn new(origin: &str, destination: &str) -> PathHash {
        PathHash { origin: origin.to_string(), destination: destination.to_string()}
    }
}

fn calculate_route_duration(addr_vec: Vec<&str>, travel_time: &HashMap<PathHash, i64>) -> i64
{
    
    let mut total_duration: i64 = 0;
    for x in 0..addr_vec.len()-1
    {
        total_duration += travel_time.get(&PathHash { origin: (addr_vec[x].to_owned()), destination: (addr_vec[x+1].to_owned()) }).unwrap().to_owned();
    }
    total_duration
    
}

fn swap(addr_vec: Vec<&str>) -> Vec<&str>
{
    let mut temp_addr_vec = addr_vec;
    let swap1: usize = rand::thread_rng().gen_range(1..temp_addr_vec.len()-1);
    let mut swap2: usize = rand::thread_rng().gen_range(1..temp_addr_vec.len()-1);
    while swap1 == swap2 {
        swap2 = rand::thread_rng().gen_range(1..temp_addr_vec.len()-1)
    }

    let addr_hold: &str = temp_addr_vec[swap1];
    temp_addr_vec[swap1] = temp_addr_vec[swap2];
    temp_addr_vec[swap2] = addr_hold;
    temp_addr_vec
}


#[tokio::main]
async fn distance_matrix_api(waypoint_vec: Vec<Waypoint>) -> DistanceMatrixResponse
{

 
    #[allow(deprecated)]
    let google_maps_client = GoogleMapsClient::new(google_maps_api__key_here);
    let distance_matrix = google_maps_client.distance_matrix(
        // Origins
        waypoint_vec.clone(),
        // Destinations
        waypoint_vec.clone(),
    ).execute().await.unwrap();

    
    distance_matrix
}

