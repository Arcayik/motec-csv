#[derive(Debug)]
struct FileInfo {
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
    unit: String,
    startbyte: usize,
    nextbyte: usize,
    length: usize,
    data: Vec<()>
}

fn main() {
    let data = std::fs::read("data/old.ld")
        .expect("Failed to read file");

    // START OF FILE
    let headerstart = get_i16(&data[8..12]);
    let datastart = get_i16(&data[12..20]);
    let _unknown = get_i16(&data[20..36]); //right b4 head
    let racetypestart = get_i16(&data[36..44]) as usize;
    let _unknown = get_i16(&data[44..74]); //bunch of bytes

    let file = FileInfo {
        device: get_utf8(&data[74..82]).to_string(),
        date: get_utf8(&data[92..126]).to_string(),
        time: get_utf8(&data[126..158]).to_string(),
        racer: get_utf8(&data[158..222]).to_string(),
        vehicle: get_utf8(&data[222..350]).to_string(),
        track: get_utf8(&data[350..478]).to_string(),
        racetype: get_utf8(&data[racetypestart .. racetypestart+64]).to_string(),
        columns: Vec::new()
    };
    //let device = get_utf8(&data[74..82]);
    //let _UNKNOWN = get_utf8(&data[82..92]);
    //let date = get_utf8(&data[92..126]);
    //let time = get_utf8(&data[126..158]);
    //let racer = get_utf8(&data[158..222]);
    //let vehicle = get_utf8(&data[222..350]);
    //let track = get_utf8(&data[350..478]); // unsure of length
    dbg!(file);

/* COLUMN HEADER NOTES
 * 0200 0100 D38702 (variant?)
 * [next entry] [data start] [num data]
 * 12001 0300 0200 3C (60) 0100 0100 0100
 *
 * [current entry] [next entry] [data start]
 * 01 [num data] "12002" 0300 0200
 * [frequency] 0100 0100 0200
 */
    let trackstart = get_i16(&data[3046..3054]);

    // HEADER


    //dbg!(datastart,device,date,time,racer,vehicle);

    let speed = shift(&vec_i16(&data[9782..10000]), 1);
    //dbg!(speed);
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

fn shift(slice: &[i16], times: usize) -> Vec<String> {
    //slice.into_iter()
    //    .map(|x| { *x.to_string(); *x.insert(x.len(),'.') })
    //    .collect::<Vec<&str>>()
    let mut out: Vec<String> = Vec::new();
    for num in slice {
        let mut val = num.to_string();
        val.insert(val.len()-times,'.');
        val.push_str("0");
        out.push(val);
    }
    out
}
