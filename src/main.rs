mod export;

#[derive(Debug)]
pub struct RaceData{
    device: String,
    date: String,
    time: String,
    driver: String,
    vehicle: String,
    track: String,
    racetype: String,
    comment: Option<String>,
    columns: Vec<Column>
}

#[derive(Debug)]
struct Column{
    name: String,
    abbv: String,
    unit: String,
    data: Vec<String>
}

fn main() {
    let data = std::fs::read("data/old.ld")
        .expect("Failed to read file");

    // START OF FILE
    let headerstart = get_usize32(&data[8..12]);
    let datastart = get_usize32(&data[12..20]);
    let _unknown = get_i16(&data[20..36]); //right b4 head
    let racetypestart = get_usize32(&data[36..44]);
    let _unknown = get_i16(&data[44..74]); //bunch of bytes

    let mut file = RaceData{
        device: get_utf8(&data[74..82]).to_string(),
        date: get_utf8(&data[92..126]).to_string(),
        time: get_utf8(&data[126..158]).to_string(),
        driver: get_utf8(&data[158..222]).to_string(),
        vehicle: get_utf8(&data[222..350]).to_string(),
        track: get_utf8(&data[350..478]).to_string(),
        racetype: get_utf8(&data[racetypestart .. racetypestart+64]).to_string(),
        comment: None,
        columns: Vec::new()
    };

/* COLUMN HEADER NOTES
 * 0200 0100 D38702 (variant?)
 * [current entry (0)] [next entry] [data start] [num data]
 * 12001 0300 0200 [frequency] 0100 0100 0100
 *
 * [current entry] [next entry] [data start]
 * 01 [num data] "12002" 0300 0200
 * [frequency] 0100 0100 0200
 */
    let trackstart = get_i16(&data[3046..3054]);

    let mut blockstart = headerstart;
    let mut numblocks = 0;
    // Jump from block to block to count
    loop {
        // Check for end of header section
        if blockstart == 0 { break; }

        let datastart = get_usize32(&data[blockstart+8 .. blockstart+12]);
        let numentries = get_usize32(&data[blockstart+12 .. blockstart+16]);
        let decimalshift = get_usize16(&data[blockstart+30 .. blockstart+34]);
        let thisblock = Column {
            name: get_utf8(&data[blockstart+32 .. blockstart+64]).to_string(),
            abbv: get_utf8(&data[blockstart+64 .. blockstart+72]).to_string(),
            unit: get_utf8(&data[blockstart+72 .. blockstart+80]).to_string(),
            data: shift(
                vec_i16(&data[datastart .. datastart+(numentries*2)]).to_owned(),
                decimalshift)
        };

        // DEBUGGING
        println!("name: {}, abbv: {}, unit: {}", thisblock.name, thisblock.abbv, thisblock.unit );
        println!("start: {datastart}\nnumentries: {numentries}\nshift: {decimalshift}");
        //

        file.columns.push(thisblock);
        numblocks += 1;
        // Get address of next block
        blockstart = get_usize32(&data[blockstart+4 .. blockstart+8]);
    }

    export::export_csv(file)
        .expect("Failed to export to csv");
}

fn vec_i16(data: &[u8]) -> Vec<i16> {
    data.chunks_exact(2)
        .into_iter()
        .map( |a| i16::from_le_bytes([a[0],a[1]]) )
        .collect::<Vec<i16>>()
}

fn get_i16(data: &[u8]) -> i16 {
    i16::from_le_bytes([data[0], data[1]])
}
fn get_i32(data: &[u8]) -> i32 {
    i32::from_le_bytes([ data[0], data[1], data[2], data[3] ])
}

fn get_utf8(slice: &[u8]) -> &str {
    core::str::from_utf8(slice).unwrap()
        .trim_matches(char::from(0))
}

fn get_usize16(data: &[u8]) -> usize {
    get_i16(data) as usize
    //usize::from_le_bytes(data.try_into().unwrap())
}
fn get_usize32(data: &[u8]) -> usize {
    get_i32(data) as usize
    //usize::from_le_bytes(data.try_into().unwrap())
}

fn shift(slice: Vec<i16>, times: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for num in slice {
        let mut val = num.to_string();
        // Check if negative
        let negative: bool = match val.find('-') {
            None => false,
            Some(idx) => {
                val.remove(idx);
                true
            }
        };
        // Insert decimal point
        if times < val.len() {
            val.insert(val.len()-times, '.');
        } else {
            for _ in 0..times { val.insert(0, '0') };
            val.insert(1, '.');
        }
        // Ensure at least two digits past decimal
        if val.split_at(val.len()-times).1.len() < 2 {
            val.push('0');
        }
        // Readd '-' if negative
        match negative {
            true => val.insert(0, '-'),
            false => continue,
        };
        // Push to output vector
        out.push(val);
    }
    out
}
