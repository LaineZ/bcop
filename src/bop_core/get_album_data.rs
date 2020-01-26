use regex::Regex;
    pub fn fix_json(album_data: &str) -> String {
        let regex = Regex::new("(?<root>url: \".+)\" \\+ \"(?<album>.+\",)");
        match regex {
            Ok(reg) => {
                let fixed_data = reg.replace(album_data, "${root}${album}");
                return fixed_data.into();
            },
            Err(e) => panic!("No matches! {}", e),
        }
    }


    pub fn get_album_data(html_code: &str) -> Option<&str> {
        let start = "var TralbumData = {";
        let stop = "};";
        
        let album_data = &html_code[html_code.find(start)? + start.len()..];
        let album_data = &album_data[..album_data.find(stop)?];
        Some(album_data)
    }