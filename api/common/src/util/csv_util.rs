use std::io::Cursor;
use anyhow::anyhow;
use csv::{Reader, Writer};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub fn to_csv<T1, T2>(headers: &Vec<T1>, body: &Vec<Vec<T2>>) -> anyhow::Result<String>
where T1: Into<String> + Serialize, T2: Into<String> + Serialize{
    let mut buf: Vec<u8> = vec![];
    {
        let mut wtr = Writer::from_writer(Cursor::new(&mut buf));
        wtr.serialize(headers)?;
        for record in body {
            wtr.serialize(record)?;
        }
        wtr.flush()?;
    }
    String::from_utf8(buf).map_err(|e| anyhow!(e))
}


pub fn csv_to_structs<T: DeserializeOwned>(csv: &str) -> anyhow::Result<Vec<T>> {
    let mut rdr = Reader::from_reader(Cursor::new(csv));

    // 读取数据并反序列化为结构体
    let mut records = Vec::new();
    for result in rdr.deserialize() {
       let record: T = result?;
       records.push(record);
    }

    Ok(records)
}

mod tests {
    use serde::Deserialize;
    use crate::util::csv_util::{csv_to_structs, to_csv};
    #[derive(Debug, Deserialize)]
    pub struct Record {
        id: String,
        name: String,
    }
    #[test]
    fn test_to_csv() {
        let data = to_csv(&vec!["id", "name"], &vec![
            vec!["123", "rock"],
            vec!["456", "john"]
        ]);
        //std::fs::write(r#"C:\rock\coding\code\my\rust\programmer-investment-research\api\tmp\test.csv"#, data.unwrap().as_bytes()).unwrap();
        let data = csv_to_structs::<Record>(&data.unwrap());
        dbg!(data);
        //dbg!(data).unwrap();
    }



    #[derive(Debug, Deserialize)]
    pub struct Income {
        pub ts_code: String,
        pub ann_date: Option<String>,
    }


    #[test]
    fn test_parse() {
        let data = r#"
ts_code,ann_date
000001.SZ,20250419
        "#;

        let mut data = String::new();
        data.push_str("ts_code,ann_date\n");
        data.push_str("000001.SZ,20250419");

        let data = csv_to_structs::<Income>(&data);
        dbg!(data);
        //dbg!(data).unwrap();
    }
}