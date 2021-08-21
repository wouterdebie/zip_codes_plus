use serde::Deserialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{collections, env};

// "Zipcode","ZipCodeType","City","State","LocationType","Lat","Long","Location","Decommisioned","TaxReturnsFiled","EstimatedPopulation","TotalWages"
#[derive(Debug, Deserialize, Clone)]
struct TempRecord {
    #[serde(rename = "Zipcode")]
    zip_code: String,
    #[serde(rename = "ZipCodeType")]
    zip_code_type: String,
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "State")]
    state: String,
    #[serde(skip)]
    location_type: String,
    #[serde(rename = "Lat")]
    latitude: Option<f64>,
    #[serde(rename = "Long")]
    longitude: Option<f64>,
    #[serde(rename = "Location")]
    location: String,
    #[serde(rename = "Decommisioned")]
    is_decommissioned: String,
    #[serde(rename = "TaxReturnsFiled")]
    tax_returns_filed: Option<u64>,
    #[serde(rename = "EstimatedPopulation")]
    estimated_population: Option<u64>,
    #[serde(rename = "TotalWages")]
    total_wages: Option<u64>,
}

impl TempRecord {
    fn to_syntax(&self) -> String {
        let mut output = String::from("Record {");
        output.push_str(&format!("zip_code: \"{}\",", self.zip_code));
        output.push_str(&format!("zip_code_type: {},", self.encoded_type()));
        output.push_str(&format!("city: \"{}\",", titlecase::titlecase(&self.city)));
        output.push_str(&format!("state: \"{}\",", self.state));
        output.push_str(&format!("coordinates: {},", self.coordinates()));
        output.push_str(&format!("is_decommissioned: {},", self.is_decommissioned));
        output.push_str(&format!("tax_returns_filed: {:?},", self.tax_returns_filed));
        output.push_str(&format!(
            "estimated_population: {:?},",
            self.estimated_population
        ));
        output.push_str(&format!("total_wages: {:?},", self.total_wages));
        output.push('}');
        output
    }

    fn encoded_type(&self) -> &'static str {
        match &self.zip_code_type[..] {
            "STANDARD" => "Type::Standard",
            "PO BOX" => "Type::PoBox",
            "UNIQUE" => "Type::Unique",
            "MILITARY" => "Type::Military",
            t => panic!("invalid ZIP code type \"{}\"", t),
        }
    }

    fn coordinates(&self) -> String {
        if let Some(ref latitude) = self.latitude {
            if let Some(ref longitude) = self.longitude {
                return format!("Some(({}_f64, {}_f64))", latitude, longitude);
            }
        }

        "None".into()
    }
}

fn main() {
    let path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    let mut zip_codes_phf = phf_codegen::Map::new();
    let mut city_to_zip_codes = phf_codegen::Map::new();
    let mut inserted_zip_codes = std::collections::HashSet::<String>::new();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data/zip_codes.csv")
        .unwrap();

    let mut city_hash_map = collections::HashMap::new();

    for result in reader.deserialize() {
        let record: TempRecord = result.unwrap();
        let syntax = record.to_syntax();
        let zip_code = record.clone().zip_code;

        // Only add zip_code if we have not inserted it before
        if !inserted_zip_codes.contains(&zip_code) {
            // Add the zip code and Record to the zip codes map
            zip_codes_phf.entry(zip_code.clone(), &syntax.to_owned());

            // Mark zip code as seen
            inserted_zip_codes.insert(zip_code.clone());

            // Create a reverse index by city.
            let city = titlecase::titlecase(&record.clone().city);
            let state = record.clone().state;

            let zip_codes_set = city_hash_map
                .entry(format!("{}, {}", city, state))
                .or_insert_with(phf_codegen::Set::new);

            zip_codes_set.entry(zip_code);
        }
    }

    for (city, records) in city_hash_map {
        city_to_zip_codes.entry(city, format!("{}", records.build()).as_str());
    }

    write!(
        &mut file,
        "static ZIP_CODES: phf::Map<&'static str, Record> = \n{};\n",
        zip_codes_phf.build()
    )
    .unwrap();

    write!(
        &mut file,
        "static CITY_MAP: phf::Map<&'static str, phf::Set<&str>> = \n{};\n",
        city_to_zip_codes.build()
    )
    .unwrap();
}
