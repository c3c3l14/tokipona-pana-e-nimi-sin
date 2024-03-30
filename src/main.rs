use rand::Rng;
use std::collections::HashMap;


fn generate_toki_pona() -> String {
    let mut result = String::new();
    // Generate a random int 2, 3, or 4 with 50% weight for 3
    let mut rng = rand::thread_rng();
    let roll_one = rng.gen_range(1..3);
    let roll_two = rng.gen_range(1..3);
    let roll = roll_one + roll_two;

    let toki_pona_dict = vec![
        String::from("a"),
        String::from("akesi"),
        String::from("ala"),
        String::from("alasa"),
        String::from("ali"),
        String::from("anpa"),
        String::from("ante"),
        String::from("anu"),
        String::from("awen"),
        String::from("epiku"),
        String::from("esun"),
        String::from("ijo"),
        String::from("ike"),
        String::from("ilo"),
        String::from("insa"),
        String::from("jaki"),
        String::from("jan"),
        String::from("jasima"),
        String::from("jelo"),
        String::from("jo"),
        String::from("kala"),
        String::from("kalama"),
        String::from("kama"),
        String::from("kasi"),
        String::from("ken"),
        String::from("kepeken"),
        String::from("kijetesantakalu"),
        String::from("kili"),
        String::from("kin"),
        String::from("kipisi"),
        String::from("kiwen"),
        String::from("ko"),
        String::from("kokosila"),
        String::from("kon"),
        String::from("ku"),
        String::from("kule"),
        String::from("kulupu"),
        String::from("kute"),
        String::from("la"),
        String::from("lanpan"),
        String::from("lape"),
        String::from("laso"),
        String::from("lawa"),
        String::from("leko"),
        String::from("len"),
        String::from("lete"),
        String::from("lili"),
        String::from("linja"),
        String::from("lipu"),
        String::from("loje"),
        String::from("lon"),
        String::from("luka"),
        String::from("lukin"),
        String::from("lupa"),
        String::from("ma"),
        String::from("mama"),
        String::from("mani"),
        String::from("meli"),
        String::from("meso"),
        String::from("mi"),
        String::from("mije"),
        String::from("misikeke"),
        String::from("moku"),
        String::from("moli"),
        String::from("monsi"),
        String::from("monsuta"),
        String::from("mu"),
        String::from("mun"),
        String::from("musi"),
        String::from("mute"),
        String::from("namako"),
        String::from("nanpa"),
        String::from("nasa"),
        String::from("nasin"),
        String::from("nena"),
        String::from("ni"),
        String::from("nimi"),
        String::from("noka"),
        String::from("oko"),
        String::from("olin"),
        String::from("ona"),
        String::from("open"),
        String::from("pakala"),
        String::from("pali"),
        String::from("palisa"),
        String::from("pan"),
        String::from("pana"),
        String::from("pi"),
        String::from("pilin"),
        String::from("pimeja"),
        String::from("pini"),
        String::from("pipi"),
        String::from("poka"),
        String::from("poki"),
        String::from("pona"),
        String::from("pu"),
        String::from("sama"),
        String::from("seli"),
        String::from("selo"),
        String::from("seme"),
        String::from("sewi"),
        String::from("sijelo"),
        String::from("sike"),
        String::from("sin"),
        String::from("sina"),
        String::from("sinpin"),
        String::from("sitelen"),
        String::from("soko"),
        String::from("sona"),
        String::from("soweli"),
        String::from("suli"),
        String::from("suno"),
        String::from("supa"),
        String::from("suwi"),
        String::from("tan"),
        String::from("taso"),
        String::from("tawa"),
        String::from("telo"),
        String::from("tenpo"),
        String::from("toki"),
        String::from("tomo"),
        String::from("tonsi"),
        String::from("tu"),
        String::from("unpa"),
        String::from("uta"),
        String::from("utala"),
        String::from("walo"),
        String::from("wan"),
        String::from("waso"),
        String::from("wawa"),
        String::from("weka"),
        String::from("wile"),
    ];
    
    //push random word from toki_pona_dict to result
    //do this as many times as the roll

    for _ in 0..roll {
        let random_index = rng.gen_range(0..toki_pona_dict.len());
        result.push_str(&toki_pona_dict[random_index]);
        result.push_str(" ");
    }
    result
}
    
    

fn main() {
    println!("{}", generate_toki_pona());
    return;
}
