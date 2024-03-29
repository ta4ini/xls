use std::collections::HashMap;

use calamine::{open_workbook, Reader, Xlsx};
use serde::{Deserialize, Serialize};

fn main() {

    let mut arr_str = Vec::new();
    for f in 1..=7 {
        arr_str.push(f);
    }

    println!("{}", get_data_from_excel("test.xlsx".to_string(), "1|2|3|4|5|6".to_string()));
    
    // if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
    //     for row in r.rows() {
    //         println!("row={:?}, row[0]={:?}", row, row[0]);
    //     }
    // }

    println!("{}", fake_bin("45385593107843568")); //3

    println!(
        "{}",
        "Hello World"
            .chars()
            .map(|f| format!("{}{}", f, f))
            .collect::<Vec<String>>()
            .join("")
    )
    //println!("Summa {}", summation(22));
    //println!("1 {}", xo("xxOo"));
    //println!("2 {}", xo("ooom"));
}

#[derive(Serialize, Deserialize, Debug)]
struct ExcelData{
    sheet_name: String,
    rows: Vec<HashMap<String,String>>
}

fn find_key_for_value(map: &HashMap<String, usize>, value: usize) -> Option<String> {
    map.iter().find_map(|(key, &val)| if val == value { Some(key.to_string()) } else { None })
}

fn get_data_from_excel(path: String, column_str: String) -> String {

    if path.is_empty() || column_str.is_empty() {
        return String::new();
    }

    let mut column_codes: HashMap<String, usize> = HashMap::new();
    column_str.split('|').for_each(|code| {
        column_codes.insert(code.to_string(), 0);
    });

    if column_codes.is_empty(){
        return String::new();
    }

    let mut excel_data_array: Vec<ExcelData> = Vec::new();

    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    for sheet_name in excel.sheet_names(){
        
        println!("{}", sheet_name);
        
        if let Ok(r) = excel.worksheet_range(&sheet_name){
            let mut rows: Vec<HashMap<String,String>> = Vec::new();
            let mut find_head_row = false;
            // println!("{}", r.rows().len());
            for row in r.rows() {

                if find_head_row {
                    let mut row_data = HashMap::new();
                    for row_idx in 0..row.len() {
                        if let Some(key) = find_key_for_value(&column_codes, row_idx){
                            row_data.insert(key, row[row_idx].to_string());
                        }
                    }

                    rows.push(row_data);
                }

                if !find_head_row && row.len() >= column_codes.len(){
                    let mut find_cell_count = 0;
                    for row_idx in 0..row.len() {
                        if column_codes.contains_key(&row[row_idx].to_string()){
                            column_codes.insert(row[row_idx].to_string(), row_idx);
                            find_cell_count = find_cell_count + 1; 
                        }
                    }

                    find_head_row = find_cell_count == column_codes.len();
                }
            }

            excel_data_array.push(ExcelData { sheet_name, rows });
        }
    }

    serde_json::to_string(&excel_data_array).unwrap()
}


fn fake_bin(s: &str) -> String {
    s.chars()
        .into_iter()
        .map(|f| if f >= '5' { '1' } else { '0' })
        .collect()
}

// fn find_short(s: &str) -> u32 {
//     //your code here
//     let mut min_length= s.len();
//     for word in s.split(' '){
//         if min_length == 0 || word.len() < min_length {min_length = word.len()}
//     }

//     min_length as u32
// }

// fn repeat_str(src: &str, count: usize) -> String {
//     (0..count).fold("".to_string(), |accum, _| accum+&src)
// }

// fn xo(string: &'static str) -> bool {
//     let mut x = 0;
//     let mut o = 0;
//     string.chars().for_each(|c| {
//         if c.to_lowercase().to_string() == "o" {o=o+1}
//         else if c.to_lowercase().to_string() == "x" {x=x+1}
//     });

//     return x == o
// }

// fn summation(n: i32) -> i32 {
//     let mut sum = 0;
//     for i in 1..=n {
//         sum = sum + i
//     }

//     sum
// }
// // fn hello_string() -> &str {
// //     return "hello world";
// // }

// fn create_str() -> String {
//     return "hello".to_string();
//     // let mut tr = String::from("");
//     // for i in 0..10000 {
//     //     tr.push_str("<tr>");
//     //     tr.push_str(&format!("{}", i+1));
//     //     for J in 0..30{
//     //         tr.push_str("<td>test</td>");
//     //     }
//     //     tr.push_str("</tr>");
//     // }
//     // return tr;
// }

// fn points(games: &[String]) -> u32 {
//     return games
//         .iter()
//         .map(|e| {
//             let (x, y) = e.split_once(':').unwrap();
//             return if x > y {
//                 3
//             } else if x == y {
//                 1
//             } else {
//                 0
//             };
//         })
//         .sum();
// }

// fn greet(name: &str, owner: &str) -> String {
//     (if name == owner {
//         "Hello boss"
//     } else {
//         "Hello guest"
//     })
//     .to_string()
// }

// fn count_sheep(sheep: &[bool]) -> u8 {
//     sheep.iter().filter(|&x| *x).count() as u8
//     // let mut cnt: u8 = 0;
//     // for n in sheep {
//     //     if *n {
//     //         cnt = cnt + 1;
//     //     }
//     // }

//     // cnt
// }
