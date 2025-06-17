use prefixfuzz::prefix::trie::Trie;
use prefixfuzz::prefix::matcher::Matcher;
// use prefixfuzz::prefix::itrie::State;


pub fn main() {
    let node_shits = vec![
        0u32, 1, 2, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 14, 16, 17, 17, 18, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27,
    ];
    let mut node_chars: Vec<Option<char>> = vec![None];
    node_chars.extend(
        vec![
            'э', 'д', 'е', 'с', 'о', 'н', ' ', 'п', 'е', 'р', 'е', 'ц', 'с', 'а', 'н', 'о', 'н',
            'и', 'с', 'о', 'н', ' ', 'п', 'е', 'р', 'е', 'ц',
        ]
        .iter()
        .map(|c| Some(*c)),
    );
    let node_payloads = vec![
        None,
        None,
        None,
        None,
        None,
        None,
        Some(2u32),
        None,
        Some(9u32),
        None,
        None,
        None,
        Some(4u32),
        None,
        Some(7u32),
        Some(8u32),
        Some(5u32),
        Some(6u32),
        None,
        None,
        None,
        Some(1u32),
        None,
        None,
        None,
        None,
        None,
        Some(3u32),
    ];
    let child_indices = vec![1u32, 2, 18, 3, 4, 13, 5, 6, 7, 8, 9, 10, 11, 12, 16, 14, 15, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27];
    
    let trie = Trie::from_internal_data(
        node_shits,
        node_chars,
        node_payloads,
        child_indices
    );
    
    let mut matcher = Matcher::new(
        "эдессон".to_string(),
        0,
        Some(2)
    );
    trie.dfs(&mut matcher);
    
    for (prefix,payload, dist) in matcher.get_results() {
        println!("{} | {} | {}", prefix, payload, dist)
    }
        

    // let mut state = State::new();
    // state.put("a".to_string(),vec![1u32, 4u32]);
    // println!("{}", state.get::<Vec<u32>>("a").unwrap()[0]);

    // state.put("CUR_PREFIX".to_string(), "".chars().collect::<Vec<char>>());
    // println!("{}", state.get::<Vec<char>>("CUR_PREFIX").unwrap().len());
}
