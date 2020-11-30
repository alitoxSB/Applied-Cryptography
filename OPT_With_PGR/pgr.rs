pub mod pgr {

    pub fn pgr_function(key: String, text: String, diference : usize) -> Vec<u8> {

        let mut new_key = Vec::with_capacity(128);
        let len_key = key.chars().count();
        let key_chars: Vec<char> = key.chars().collect();

        let seed = ['c', 'h', 'a', 'p', 'o', 'r', 'o', 'o'];
        let mut seed_expand = shift_seed(seed, key, text);

        let mut n = 0;

        while n < diference {
            new_key.push(seed_expand[n]);
            n = n + 1;
        }
        n = 0;
        while n < len_key{
            new_key.push(key_chars[n]);
            n = n + 1;
        }
        let key_as_String : String = new_key.into_iter().collect();
        let key_expand = key_as_String.as_bytes();
        return Vec::from(key_expand);
    }

    pub fn shift_seed(seed: [char; 8], key: String, text: String) -> Vec<char> {

        let diference = text.chars().count() - key.chars().count();
        let mut new_seed = Vec::with_capacity(128);
        let mut n = 0;

        if diference < 5 {
            while n < diference {
                new_seed.push(seed[7 - n]);
                n = n + 1;
            }

        } else {
            new_seed.push(seed[7]);
            new_seed.push(seed[0]);
            new_seed.push(seed[6]);
            new_seed.push(seed[5]);
            new_seed.push(seed[4]);
            new_seed.push(seed[2]);
            new_seed.push(seed[1]);
            new_seed.push(seed[3]);
        }
        return new_seed;
    }

}