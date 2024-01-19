use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref OUTFIT_MAP: HashMap<&'static str, HashMap<&'static str, i32>> = {
        let mut map = HashMap::new();

        let mut chloe_map = HashMap::new();
        chloe_map.insert("monday", 10);
        chloe_map.insert("tuesday", 4);
        chloe_map.insert("wednesday", 9);
        chloe_map.insert("thursday", 10);
        chloe_map.insert("friday", 4);
        chloe_map.insert("saturday", 9);
        chloe_map.insert("sunday", 9);
        map.insert("chloe", chloe_map);

        let mut nora_map = HashMap::new();
        nora_map.insert("monday", 6);
        nora_map.insert("tuesday", 7);
        nora_map.insert("wednesday", 5);
        nora_map.insert("thursday", 6);
        nora_map.insert("friday", 7);
        nora_map.insert("saturday", 5);
        nora_map.insert("sunday", 5);
        map.insert("cora", nora_map);

        let mut lindsey_map = HashMap::new();
        lindsey_map.insert("monday", 9);
        lindsey_map.insert("tuesday", 7);
        lindsey_map.insert("wednesday", 8);
        lindsey_map.insert("thursday", 9);
        lindsey_map.insert("friday", 7);
        lindsey_map.insert("saturday", 8);
        lindsey_map.insert("sunday", 8);
        map.insert("lindsey", lindsey_map);

        let mut aubrey_map = HashMap::new();
        aubrey_map.insert("monday", 14);
        aubrey_map.insert("tuesday", 12);
        aubrey_map.insert("wednesday", 13);
        aubrey_map.insert("thursday", 14);
        aubrey_map.insert("friday", 12);
        aubrey_map.insert("saturday", 13);
        aubrey_map.insert("sunday", 13);
        map.insert("aubrey", aubrey_map);

        let mut penelope_map = HashMap::new();
        penelope_map.insert("monday", 7);
        penelope_map.insert("tuesday", 10);
        penelope_map.insert("wednesday", 3);
        penelope_map.insert("thursday", 7);
        penelope_map.insert("friday", 10);
        penelope_map.insert("saturday", 3);
        penelope_map.insert("sunday", 3);
        map.insert("penelope", penelope_map);

        let mut samantha_map = HashMap::new();
        samantha_map.insert("monday", 1);
        samantha_map.insert("tuesday", 1);
        samantha_map.insert("wednesday", 1);
        samantha_map.insert("thursday", 1);
        samantha_map.insert("friday", 1);
        samantha_map.insert("saturday", 1);
        samantha_map.insert("sunday", 1);
        map.insert("samantha", samantha_map);

        let mut amber_map = HashMap::new();
        amber_map.insert("monday", 7);
        amber_map.insert("tuesday", 1);
        amber_map.insert("wednesday", 6);
        amber_map.insert("thursday", 7);
        amber_map.insert("friday", 2);
        amber_map.insert("saturday", 1);
        amber_map.insert("sunday", 6);
        map.insert("amber", amber_map);

        let mut lauren_map = HashMap::new();
        lauren_map.insert("monday", 6);
        lauren_map.insert("tuesday", 7);
        lauren_map.insert("wednesday", 5);
        lauren_map.insert("thursday", 6);
        lauren_map.insert("friday", 7);
        lauren_map.insert("saturday", 5);
        lauren_map.insert("sunday", 5);
        map.insert("lauren", lauren_map);

        let mut ms_rose_map = HashMap::new();
        ms_rose_map.insert("monday", 3);
        ms_rose_map.insert("tuesday", 1);
        ms_rose_map.insert("wednesday", 2);
        ms_rose_map.insert("thursday", 3);
        ms_rose_map.insert("friday", 1);
        ms_rose_map.insert("saturday", 2);
        ms_rose_map.insert("sunday", 2);
        map.insert("ms. rose", ms_rose_map);

        let mut riley_map = HashMap::new();
        riley_map.insert("monday", 8);
        riley_map.insert("tuesday", 10);
        riley_map.insert("wednesday", 11);
        riley_map.insert("thursday", 9);
        riley_map.insert("friday", 8);
        riley_map.insert("saturday", 10);
        riley_map.insert("sunday", 9);
        map.insert("riley", riley_map);

        let mut emily_map = HashMap::new();
        emily_map.insert("monday", 1);
        emily_map.insert("tuesday", 3);
        emily_map.insert("wednesday", 1);
        emily_map.insert("thursday", 2);
        emily_map.insert("friday", 3);
        emily_map.insert("saturday", 1);
        emily_map.insert("sunday", 2);
        map.insert("emily", emily_map);

        let mut autumn_map = HashMap::new();
        autumn_map.insert("monday", 5);
        autumn_map.insert("tuesday", 6);
        autumn_map.insert("wednesday", 4);
        autumn_map.insert("thursday", 5);
        autumn_map.insert("friday", 6);
        autumn_map.insert("saturday", 4);
        autumn_map.insert("sunday", 4);
        map.insert("autumn", autumn_map);

        let mut chris_map = HashMap::new();
        chris_map.insert("monday", 1);
        chris_map.insert("tuesday", 2);
        chris_map.insert("wednesday", 1);
        chris_map.insert("thursday", 2);
        chris_map.insert("friday", 1);
        chris_map.insert("saturday", 2);
        chris_map.insert("sunday", 2);
        map.insert("chris", chris_map);

        let mut josh_map = HashMap::new();
        josh_map.insert("monday", 1);
        josh_map.insert("tuesday", 1);
        josh_map.insert("wednesday", 1);
        josh_map.insert("thursday", 1);
        josh_map.insert("friday", 1);
        josh_map.insert("saturday", 1);
        josh_map.insert("sunday", 1);
        map.insert("josh", josh_map);

        let mut cameron_map = HashMap::new();
        cameron_map.insert("monday", 2);
        cameron_map.insert("tuesday", 3);
        cameron_map.insert("wednesday", 2);
        cameron_map.insert("thursday", 3);
        cameron_map.insert("friday", 2);
        cameron_map.insert("saturday", 3);
        cameron_map.insert("sunday", 3);
        map.insert("cameron", cameron_map);

        let mut ryan_map = HashMap::new();
        ryan_map.insert("monday", 1);
        ryan_map.insert("tuesday", 1);
        ryan_map.insert("wednesday", 1);
        ryan_map.insert("thursday", 1);
        ryan_map.insert("friday", 1);
        ryan_map.insert("saturday", 1);
        ryan_map.insert("sunday", 1);
        map.insert("ryan", ryan_map);

        let mut imre_map = HashMap::new();
        imre_map.insert("monday", 1);
        imre_map.insert("tuesday", 2);
        imre_map.insert("wednesday", 1);
        imre_map.insert("thursday", 2);
        imre_map.insert("friday", 1);
        imre_map.insert("saturday", 2);
        imre_map.insert("sunday", 2);
        map.insert("imre", imre_map);

        let mut mr_lee_map = HashMap::new();
        mr_lee_map.insert("monday", 1);
        mr_lee_map.insert("tuesday", 1);
        mr_lee_map.insert("wednesday", 1);
        mr_lee_map.insert("thursday", 1);
        mr_lee_map.insert("friday", 1);
        mr_lee_map.insert("saturday", 1);
        mr_lee_map.insert("sunday", 1);
        map.insert("mr. lee", mr_lee_map);

        let mut mc_map = HashMap::new();
        mc_map.insert("monday", 15);
        mc_map.insert("tuesday", 16);
        mc_map.insert("wednesday", 17);
        mc_map.insert("thursday", 14);
        mc_map.insert("friday", 15);
        mc_map.insert("saturday", 16);
        mc_map.insert("sunday", 14);
        map.insert("mc", mc_map);

        let mut grayson_map = HashMap::new();
        grayson_map.insert("monday", 2);
        grayson_map.insert("tuesday", 3);
        grayson_map.insert("wednesday", 2);
        grayson_map.insert("thursday", 3);
        grayson_map.insert("friday", 2);
        grayson_map.insert("saturday", 3);
        grayson_map.insert("sunday", 3);
        map.insert("grayson", grayson_map);

        let mut sebastian_map = HashMap::new();
        sebastian_map.insert("monday", 2);
        sebastian_map.insert("tuesday", 1);
        sebastian_map.insert("wednesday", 2);
        sebastian_map.insert("thursday", 1);
        sebastian_map.insert("friday", 2);
        sebastian_map.insert("saturday", 1);
        sebastian_map.insert("sunday", 1);
        map.insert("sebastian", sebastian_map);

        map
    };
}
