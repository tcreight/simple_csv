extern crate csv;
extern crate serde;

use std::io;
use std::result::Result;
use std::error::Error;
use serde::Deserialize;
use serde::Serialize;

//Storage for deserialized CSV input records.

#[derive(Debug, Deserialize)]
pub struct Record {
    temp: String,
    rh: String,
}

impl Record {
    pub fn new(temp: String, rh: String) -> Self {
        Record {
            temp,
            rh
        }
    }
}

//Storage for serialized CSV output records.
#[derive(Debug, Serialize)]
pub struct State {
    pub heat: String,
    pub cool: String
}

impl State {
    pub fn new() -> Self {
        Self {
            heat: String::new(),
            cool: String::new(),
        }
    }

    //Write output to CSV
    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());
        wtr.serialize(self)?;
        wtr.flush()?;
        Ok(())
    }
}

//Write headers
pub fn write_header() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new().from_writer(io::stdout());
    wtr.serialize(("Heat", "Cool"))?;
    wtr.flush()?;
Ok(())
}

//Determine equipment state.
fn get_state(temperature: f32, humidity: f32) -> State {
    let mut state = State::new();
    
    if temperature >= 70.0 {
            state.heat = String::from("off");
            state.cool = String::from("on");   
        
    } else if humidity > 75.0 && temperature < 80.0 {
            state.heat = String::from("on");
            state.cool = String::from("off"); 
        
    } else if temperature <= 60.0 {
        state.heat = String::from("on");
        state.cool = String::from("off"); 
        
    } else {
        state.heat = String::from("off");
        state.cool = String::from("off"); 
    }

    state
}
       
    fn read() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().from_path("input.csv")?;
    //Read in
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        let temperature: f32 = record.temp.parse::<f32>().unwrap();
        let humidity: f32 = record.rh.parse::<f32>().unwrap();

        //Determine eqipment status and write output.
        let state =  get_state(temperature, humidity);
        let _ = state.write();
        }
    Ok(())
    }


fn main() {
    write_header().unwrap(); 
    read().unwrap();
    
}
   