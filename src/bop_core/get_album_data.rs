use regex::Regex;
use crate::structs;


    pub fn fix_json(album_data: &str) -> String {
        let fixed_data;
        let fixed_data_json;


        let regex = Regex::new("(?P<root>url: \".+)\" \\+ \"(?P<album>.+\",)");
        
        match regex {
            Ok(reg) => {
                fixed_data = reg.replace_all(album_data, "$root$album");
            },
            Err(e) => panic!("No matches! {}", e),
        }

        let regex = Regex::new("    (?P<property>[a-zA-Z_]+):");
        
        match regex {
            Ok(reg) => {
               fixed_data_json = reg.replace_all(&fixed_data, "\"$property\":");
            },
            Err(e) => panic!("No matches! {}", e),
        }

        let regex = Regex::new("// .*");
        
        match regex {
            Ok(reg) => {
                let remove_comments = reg.replace_all(&fixed_data_json, "");
                remove_comments.into()
            },
            Err(e) => panic!("No matches! {}", e),
        }
    }


    pub fn get_album_data(html_code: &str) -> Option<&str> {
        let start = "var TralbumData = {";
        let stop = "};";
        
        let album_data = &html_code[html_code.find(start)? + start.len() - 1..];
        let album_data = &album_data[..album_data.find(stop)? + 1];
        Some(album_data)
    }

    pub async fn get_tag_data(tags: String, page: i32) -> structs::struct_json_discover::Root
    {
        let client = reqwest::Client::new();

        let request_body = format!("{{\"filters\":{{ \"format\":\"all\",\"location\":0,\"sort\":\"pop\",\"tags\":[\"{}\"] }},\"page\":\"{}\"}}", tags, page);
        let res = client.post("https://bandcamp.com/api/hub/2/dig_deeper")
                                            .body(request_body)
                                            .send()
                                            .await;
        match res {
            Ok(value) => {
                return serde_json::from_str(value.text().await.unwrap().as_str()).unwrap();
            }
            Err(error) => {
                panic!("пизда {:#?}", error);
            }
        }
    }