#[derive(Debug)]
struct RaceData{
    device: String,
    date: String,
    time: String,
    racer: String,
    vehicle: String,
    track: String,
    racetype: String,
    columns: Vec<Column>
}

#[derive(Debug)]
struct Column{
    name: String,
    abbv: String,
    unit: String,
    //startbyte: usize,
    //nextbyte: usize,
    entries: usize,
    data: Vec<String>
}

fn main() {
    let data = std::fs::read("data/old.ld")
        .expect("Failed to read file");

    // START OF FILE
    let headerstart = get_usize(&data[8..12]);
    let datastart = get_usize(&data[12..20]);
    let _unknown = get_i16(&data[20..36]); //right b4 head
    let racetypestart = get_usize(&data[36..44]);
    let _unknown = get_i16(&data[44..74]); //bunch of bytes

    let mut file = RaceData{
        device: get_utf8(&data[74..82]).to_string(),
        date: get_utf8(&data[92..126]).to_string(),
        time: get_utf8(&data[126..158]).to_string(),
        racer: get_utf8(&data[158..222]).to_string(),
        vehicle: get_utf8(&data[222..350]).to_string(),
        track: get_utf8(&data[350..478]).to_string(),
        racetype: get_utf8(&data[racetypestart .. racetypestart+64]).to_string(),
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

    // Jump from block to block to count
    let mut blockstart = get_usize(&data[headerstart+4 .. headerstart+8]);
    let mut numblocks = 1;
    loop {
        blockstart = get_usize(&data[blockstart+4 .. blockstart+8]);

        let datastart = get_usize(&data[8..12]);
        let numentries = get_usize(&data[blockstart+12 .. blockstart+20]);
        let decimalshift = get_usize(&data[blockstart+30 .. blockstart+32]);
        let thisblock = Column {
            name: get_utf8(&data[blockstart+32 .. blockstart+64]).to_string(),
            abbv: get_utf8(&data[blockstart+64 .. blockstart+72]).to_string(),
            unit: get_utf8(&data[blockstart+72 .. blockstart+80]).to_string(),
            entries: get_usize(&data[blockstart+12 .. blockstart+20]),
            data: shift(
                vec_i16(&data[datastart .. datastart+(numentries*2)]).to_owned(),
                decimalshift)
        };
        file.columns.push(thisblock);

        if blockstart == 0 && numblocks > 0 { break; }
        numblocks += 1;
    }

    dbg!(file);
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

fn get_utf8(slice: &[u8]) -> &str {
    core::str::from_utf8(slice).unwrap()
        .trim_matches(char::from(0))
}

fn get_usize(data: &[u8]) -> usize {
    get_i16(data) as usize
    //usize::from_le_bytes(data[index..index+8].try_into().unwrap())
}

fn shift(slice: Vec<i16>, times: usize) -> Vec<String> {
    //slice.into_iter()
    //    .map(|x| { *x.to_string(); *x.insert(x.len(),'.') })
    //    .collect::<Vec<&str>>()
    dbg!(&times);
    let mut out: Vec<String> = Vec::new();
    //for num in slice {
    //    let mut val = num.to_string();
    //    val.insert(val.len()-times,'.');
    //    val.push_str("0");
    //    out.push(val);
    //}
    if times < 1 {
        for num in slice {
            out.push(num.to_string());
        }
    } else {
        for num in slice {
            out.push(format!("{:.6}",(num as f32 / (10*times) as f32).to_string()));
        }
    }
    out
}
