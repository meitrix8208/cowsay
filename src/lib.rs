
extern crate clap;
extern crate phf;
extern crate rand;
#[macro_use]
extern crate rust_embed;

use phf::phf_map;
use std::fs::File;
use std::io::Read;
use std::str;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/cows/"]
struct Asset;

struct CowBubble {
    sleft: &'static str,
    sright: &'static str,
    topleft: &'static str,
    midleft: &'static str,
    botleft: &'static str,
    topright: &'static str,
    midright: &'static str,
    botright: &'static str,
}

static SAY_COW_BUBBLE: CowBubble = CowBubble {
    sleft: "<",
    sright: ">",
    topleft: "/",
    midleft: "|",
    botleft: "\\",
    topright: "\\",
    midright: "|",
    botright: "/",
};

static THINK_COW_BUBBLE: CowBubble = CowBubble {
    sleft: "(",
    sright: ")",
    topleft: "(",
    midleft: "(",
    botleft: "(",
    topright: ")",
    midright: ")",
    botright: ")",
};

static EYES: phf::Map<&'static str, &'static str> = phf_map! {
    "borg" =>  "==",
    "dead" => "xx",
    "greedy" => "$$",
    "paranoid" => "@@",
    "stoned" => "**",
    "tired" => "--",
    "wired" => "OO",
    "youthful" => "..",
    "default" => "oo"
};

pub fn list_cows() -> Vec<String> {
    Asset::iter()
        .map(|file| file.as_ref().replace(".cow", ""))
        .collect::<Vec<String>>()
}

pub fn get_eyes(input: &str) -> &str {
    EYES.get(input).unwrap_or(&input)
}

pub fn format_cow(
    message: &String,
    cow: &String,
    width: usize,
    think: bool,
    wrap: bool,
    eyes: &str,
    tongue: &str,
) -> String {
    let mut cowbody = String::new();
    let voice = if think { "o" } else { "\\" };

    match cow.contains(".cow") {
        true => {
            let mut f = File::open(&cow).unwrap();
            f.read_to_string(&mut cowbody)
                .expect(&format!("Couldn't read cowfile {}", cow));
        }
        false => {
            let fmt = format!("{}.cow", &cow);
            let asset = Asset::get(&fmt).unwrap();
            cowbody = str::from_utf8(&asset.data).unwrap().to_string();
        }
    }

    let bubble = make_bubble(message, width, think, wrap);
    let animal = format_animal(cowbody, voice, eyes, tongue);
    format!("{}\n{}", bubble, animal)
}

fn format_animal(s: String, thoughts: &str, eyes: &str, tongue: &str) -> String {
    s.split("\n")
        .filter(|&x| !x.starts_with("##") && !x.contains("EOC"))
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .replace("$eyes", eyes)
        .replace("$thoughts", thoughts)
        .replace("$tongue", tongue)
        .replace("\\\\", "\\")
        .replace("\\@", "@")
}

fn make_bubble(message: &String, width: usize, think: bool, wrap: bool) -> String {
    let mut result = Vec::new();
    let mut top = vec![" "];
    let mut bottom = vec![" "];
    let topc = "_";
    let bottomc = "-";
    let pad = ' ';
    let cowb = if think {
        &THINK_COW_BUBBLE
    } else {
        &SAY_COW_BUBBLE
    };

    // Linewrap
    let mut index = 0;
    if wrap {
        loop {
            if index + width >= message.len() {
                break;
            }

            let localwidth;
            let mut subindex = index + width;
            'b: loop {
                match (&message[index..subindex]).ends_with(" ") {
                    true => {
                        localwidth = subindex - index;
                        break 'b;
                    }
                    false => {
                        subindex -= 1;
                    }
                }
            }
            let slice = &message[index..index + localwidth];
            result.push(slice.to_string());
            index += localwidth;
        }
    }
    let slice = &message[index..];
    result.push(slice.to_string());

    // Bookend lines with bubble chars
    let mut longest = 0;
    let reslen = result.len() - 1;
    for (index, line) in result.iter_mut().enumerate() {
        match index {
            0 => match reslen {
                0 | 1 => *line = vec![cowb.sleft, line, cowb.sright].join(" "),
                _ => *line = vec![cowb.topleft, line, cowb.topright].join(" "),
            },
            x if x < reslen => *line = vec![cowb.midleft, line, cowb.midright].join(" "),
            y if y == reslen => match reslen {
                1 => *line = vec![cowb.sleft, line, cowb.sright].join(" "),
                _ => *line = vec![cowb.botleft, line, cowb.botright].join(" "),
            },
            _ => panic!("Whoops!"),
        }
        if line.len() > longest {
            longest = line.len();
        }
    }

    // Pad to longest line
    for line in &mut result {
        let mut padding = longest - line.len();
        let linelen = line.len();
        loop {
            match padding > 0 {
                false => break,
                true => {
                    line.insert(linelen - 1, pad);
                    padding -= 1;
                }
            };
        }
    }

    let mut top_bottom = longest - 2;
    loop {
        match top_bottom > 0 {
            false => break,
            true => {
                top.push(topc);
                bottom.push(bottomc);
                top_bottom -= 1;
            }
        }
    }
    result.insert(0, top.join(""));
    result.push(bottom.join(""));
    result.join("\n")
}

#[cfg(test)]
mod tests {
    use list_cows;
    #[test]
    fn it_list_cows() {
        let cows = list_cows();
        println!("{:?}", cows);
    }
    use rand::seq::SliceRandom;

    #[test]
    fn random_cows() {
        let cows = list_cows();
        let cow = cows.choose(&mut rand::thread_rng()).unwrap().to_owned();
        println!("{:?}", cow);
    }

    use EYES;
    #[test]
    fn eyes_are_found() {
        let eye = EYES.get("borg").unwrap().to_string();

        assert_eq!(eye, "==");
    }
}
