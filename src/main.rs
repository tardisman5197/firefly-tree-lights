use rand::Rng;
use std::process;
use std::fs::File;
use std::time::Instant;
use std::io::{BufWriter, Write};
use std::cmp::Ordering;
extern crate clap;
use clap::{Arg, App};

// MAX_COUNTDOWN is the interval of ticks
// between a Fly flashing.
const MAX_COUNTDOWN: &str = "40";
// NUDGE_VALUE is the number of ticks which
// a Fly will move towards its neighbouring
// Fly's flash.
const NUDGE_VALUE: &str = "1";
// NO_OF_NEIGHBOURS is the initial number of 
// neighbours which each Fly has. 
const NO_OF_NEIGHBOURS: &str = "5";
// NO_OF_TICKS is the maximum number of ticks
// the simulation runs for.
const NO_OF_TICKS: &str = "60000";
// SYNC_STOP is the number of fully synced flashes
// the swarm performs before stopping.
const SYNC_STOP: &str = "10";


fn main() {
    // Read in arguments for the simulation
    let matches = App::new("Firefly Tree Lights")
        .version("1.0")
        .author("Ed")
        .about("Creates a Firefly synchronisation tree light animation")
        .arg(Arg::with_name("Input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("The path to the input csv")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("Output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("The path to where the output csv should be stored")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Logs with verbose messages"))
        .arg(Arg::with_name("Nudge Value")
            .short("n")
            .long("nudge")
            .value_name("NO OF TICKS")
            .help("The number of ticks that a Firefly can be nudged closer to other Fireflies")
            .takes_value(true))
        .arg(Arg::with_name("Max Countdown")
            .short("c")
            .long("countdown")
            .value_name("NO OF TICKS")
            .help("The number of ticks between a Firefly's flash")
            .takes_value(true))
        .arg(Arg::with_name("Initial Neighbours")
            .short("nb")
            .long("neighbours")
            .value_name("NO OF NEIGHBOURS")
            .help("The initial number of neighbours each Firefly starts with")
            .takes_value(true))
        .arg(Arg::with_name("Max Ticks")
            .short("t")
            .long("ticks")
            .value_name("NO OF TICKS")
            .help("The maximum number of ticks the simulation can run for")
            .takes_value(true))
        .arg(Arg::with_name("Sync Stop")
            .short("s")
            .long("sync")
            .value_name("NO OF SYNCS")
            .help("The number of totally synced flashes to execute before stopping the simulation")
            .takes_value(true))
        .get_matches();

    let input = matches.value_of("Input").unwrap_or("./input/matts-tree.csv");
    let output = matches.value_of("Output").unwrap_or("./output/out.csv");
    let nudge_value = matches.value_of("Nudge Value").unwrap_or(NUDGE_VALUE).parse::<i16>().unwrap();
    let no_of_neighbours = matches.value_of("Initial Neighbours").unwrap_or(NO_OF_NEIGHBOURS).parse::<usize>().unwrap();
    let no_of_ticks = matches.value_of("Max Ticks").unwrap_or(NO_OF_TICKS).parse::<usize>().unwrap();
    let max_countdown = matches.value_of("Max Countdown").unwrap_or(MAX_COUNTDOWN).parse::<i16>().unwrap();
    let sync_stop = matches.value_of("Sync Stop").unwrap_or(SYNC_STOP).parse::<i16>().unwrap();
    let verbose = matches.is_present("verbose");

    // Read the input file and find each Fly's
    // closest Flies
    let mut flies = read_in_flies(input, max_countdown, nudge_value, no_of_neighbours);
    flies = calc_neighbours(flies);

    print!("Input File:               {}
Output File:              {}
No of Flies:              {}
Max Countdown:            {}
Nudge Value:              {}
Initial No of Neighbours: {}
Max Ticks:                {}
Sync Stop Count:          {}\n",
        input,
        output,
        flies.len(),
        max_countdown,
        nudge_value,
        no_of_neighbours,
        no_of_ticks,
        sync_stop,
    );
    println!("======");

    // Create the output file
    let out = match File::create(output) {
        Ok(out) => out,
        Err(e) => {
            println!("{:?}", e);
            process::exit(1);
        }
    };
    let mut buf = BufWriter::new(out);
    
    // Create swarm 
    let mut swarm: Swarm = Swarm::new();
    swarm.set_flies(flies);
    
    println!("Swarm Started");
    let start = Instant::now();
    // Run sim until synced or timeout
    let mut sync_count = 0;
    for i in 0..no_of_ticks {
        // Simulate a tick for each Fly
        swarm.tick();
        // Write the frame number to the output file
        write!(buf, "{},",i).unwrap();
        
        // Get a vector of all of the Flies which are
        // currently flashing on this tick
        let lit = swarm.lit_flies();
        // Calculate the Red and Green values for the Fly's
        // flash based on how many flies are currently lit.
        // Red = All flies, Green = One Fly
        let r = 255 * lit.len()/swarm.flies.len();
        let g = 255 - r;
        // Write each Fly's colour value to the output file
        swarm.flies.iter().enumerate().for_each(|(i, fly)| {
            if fly.lit() {
                write!(buf, "{},{},0", r, g).unwrap();
            } else {
                write!(buf, "0,0,0").unwrap();
            }
            // Add a trailing comma if it is not
            // the final fly in the list.
            if i < swarm.flies.len()-1 {
                write!(buf, ",").unwrap();
            }
        });

        // End the current frame in the output file with
        // a \n and flush the frame to the file.
        write!(buf, "\n").unwrap();
        buf.flush().unwrap();

        if verbose {
            println!("{} - Lit: {:?}", i, lit)
        }

        // Check if all of the Flies are currently lit
        // If they are this means they are fully synced.
        if lit.len() == swarm.flies.len() {
            // Keep track of the number of fully synced
            // flashes have occurred and stop after the
            // specified number of flashes.
            sync_count += 1;
            if sync_count >= sync_stop {
                println!("Sync Stop - Ticks: {}", i);
                break
            }
        }
    }
    let duration = start.elapsed();
    println!("Swarm Stopped - Exec Time: {:?}", duration)
}

// read_in_flies takes a filepath to the input coordinates
// and creates a Fly for each coord.
fn read_in_flies(filepath: &str, max_countdown: i16, nudge_value: i16, no_of_neighbours: usize) -> Vec<Fly> {
    let input = std::fs::read_to_string(filepath).unwrap();
    input
        .lines()
        .map(|line| {
            let val: Vec<f64> = line.split(",")
                .map(|coord| coord.parse::<f64>().unwrap())
                .collect();
            (val[0], val[1], val[2])
        })
        .fold(Vec::new(), |mut flies, coords| {
            flies.push(Fly::new(coords, max_countdown, nudge_value, no_of_neighbours));
            flies
        })
}

// calc_neighbours takes in a vector of Flies and 
// sets each Fly's neighbours to an ordered list based
// on their distance between each other.
fn calc_neighbours(mut flies: Vec<Fly>) -> Vec<Fly> {
    let flies_cpy = flies.clone();
    // Loop through each fly
    flies.iter_mut().enumerate().for_each(|(f_index, f)| {
        // Create a list of (Fly Index, Distance to current Fly)
        let mut distances: Vec<(usize, f64)> = flies_cpy.iter().enumerate()
            .map(|(o_index, other)| (o_index, other.to(f.position)))
            .collect();

        // Sort the list by closest Fly first
        distances.sort_by(|a, b| {
            if a.1 < b.1 {
                Ordering::Less
            } else if a.1 == b.1 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }       
        });

        // Remove the current Fly from the list of neighbours.
        let neighbours = distances.iter()
            .fold(Vec::new(), | mut neighbours, (index, distance)| {

            if *index != f_index {
                neighbours.push((*index, *distance));
            }

            neighbours
        });
        // Set the current Fly's neighbours to the sorted
        // list.
        f.neighbours = neighbours
    });
    // Return the neighbourified flies list
    flies
}

// Swarm houses all of the Flies in the
// simulation.
#[derive(Debug, Clone)]
struct Swarm {
    flies: Vec<Fly>
}

impl Swarm {
    fn new() -> Swarm {
        Swarm{flies: Vec::new()}
    }

    fn set_flies(&mut self, flies: Vec<Fly>) {
        self.flies = flies;
    }

    // lit_flies returns a list of the indexes of 
    // each Fly in the Swarm which is currently lit.
    fn lit_flies(&self) -> Vec<usize> {
        self.flies.iter().enumerate().fold(Vec::new(), |mut lit, (i, f)| {
            if f.lit() {lit.push(i)};
            lit
        })
    }

    // tick executes a single tick for each of the
    // Flies in the Swarm.
    fn tick(&mut self) {
        // Tick each fly in the warm
        self.flies.iter_mut().for_each(|f| f.tick());

        // Nudge each fly if needed
        for i in 0..self.flies.len() {
            // Do not nudge the fly if it is
            // currently lit
            if self.flies[i].lit() {
                continue
            }
            // Nudge the current fly towards the
            // brightest light (the closest fly currently
            // lit)
            let neighbours = self.flies[i].neighbours.clone();
            for j in 0..self.flies[i].no_of_neighbours {
                if self.flies[neighbours[j].0].lit(){
                    self.flies[i].nudge();
                    break;
                }
            }
        }
    }
}

// Fly flashes periodically and has the ability
// to watch for other flies flashing around them
// and aims to sync up with them.
#[derive(Debug, Clone)]
struct Fly {
    countdown: i16,
    max_countdown: i16,
    nudge_value: i16,
    position: (f64, f64, f64),
    neighbours: Vec<(usize, f64)>,
    find_new_neighbours: bool,
    no_of_neighbours: usize
}

impl Fly {
    // new returns an instance of Fly with the
    // coordinates provided and a random countdown
    // value between 0 and MAX_COUNTDOWN to begin.
    fn new(pos: (f64, f64, f64), max_countdown: i16, nudge_value: i16, no_of_neighbours: usize) -> Fly {
        let mut rng = rand::thread_rng();
        Fly{
            countdown: rng.gen_range(0..max_countdown),
            max_countdown: max_countdown,
            nudge_value: nudge_value,
            position: pos,
            neighbours: vec!(),
            no_of_neighbours: no_of_neighbours,
            find_new_neighbours: true,
        }
    }

    // tick counts down the countdown timer
    // of the Fly. If the Fly has not been
    // nudged since it's last flash then the
    // Fly is in sync with its neighbours, so it
    // will expand its neighbourhood.
    fn tick(&mut self) {
        if self.countdown <= 0 {
            self.countdown = self.max_countdown;
            // Grow neighbours if in sync
            if self.find_new_neighbours {
                self.no_of_neighbours += 1;
                if self.no_of_neighbours >= self.neighbours.len() {
                    self.no_of_neighbours = self.neighbours.len()-1;
                }
            }
        } else {
            self.countdown -= 1;
        }
        self.find_new_neighbours = true;
    }

    // lit returns true if the Fly is currently
    // at the end of its countdown.
    fn lit(&self) -> bool {
        self.countdown <= 0
    }

    // nudge should be called when one of the
    // Fly's neighbours is currently lit. The
    // Fly will shift its countdown towards the
    // the current tick. So it will delay its
    // next flash if it has just happened or 
    // reduce the time to the next flash if it
    // has been a while since the Fly has flashed.
    fn nudge(&mut self) {
        if (self.countdown) > self.max_countdown/2 {
            self.countdown += self.nudge_value;
        } else {
            self.countdown -= self.nudge_value;
        }
        self.find_new_neighbours = false;
    }

    // to calculates the distance between the
    // Fly's position and the coordinates provided.
    fn to(&self, b: (f64, f64, f64)) -> f64 {
        let dx = self.position.0-b.0;
        let dy = self.position.1-b.1;
        let dz = self.position.2-b.2;
        ((dx*dx) + (dy*dy) + (dz*dz)).sqrt()
    }
}