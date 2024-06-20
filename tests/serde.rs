#![cfg(feature = "serde_impl")]

use hashlink::{LinkedHashMap, LinkedHashSet, LruCache};
use rustc_hash::FxHasher;
use serde_test::{assert_tokens, Token};
use std::hash::BuildHasherDefault;

#[cfg(target_pointer_width = "64")]
fn token_usize(t: usize) -> Token {
    Token::U64(t as u64)
}
#[cfg(target_pointer_width = "32")]
fn token_usize(t: usize) -> Token {
    Token::U32(t as u32)
}
#[cfg(target_pointer_width = "16")]
fn token_usize(t: usize) -> Token {
    Token::U16(t as u16)
}

#[test]
fn map_serde_tokens_empty() {
    let map = LinkedHashMap::<char, u32>::new();

    assert_tokens(&map, &[Token::Map { len: Some(0) }, Token::MapEnd]);
}

#[test]
fn map_serde_tokens() {
    let mut map = LinkedHashMap::new();
    map.insert('a', 10);
    map.insert('b', 20);
    map.insert('c', 30);

    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(3) },
            Token::Char('a'),
            Token::I32(10),
            Token::Char('b'),
            Token::I32(20),
            Token::Char('c'),
            Token::I32(30),
            Token::MapEnd,
        ],
    );
}

#[test]
fn map_serde_tokens_empty_generic() {
    let map = LinkedHashMap::<char, u32, BuildHasherDefault<FxHasher>>::default();

    assert_tokens(&map, &[Token::Map { len: Some(0) }, Token::MapEnd]);
}

#[test]
fn map_serde_tokens_generic() {
    let mut map = LinkedHashMap::<char, i32, BuildHasherDefault<FxHasher>>::default();
    map.insert('a', 10);
    map.insert('b', 20);
    map.insert('c', 30);

    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(3) },
            Token::Char('a'),
            Token::I32(10),
            Token::Char('b'),
            Token::I32(20),
            Token::Char('c'),
            Token::I32(30),
            Token::MapEnd,
        ],
    );
}

#[test]
fn set_serde_tokens_empty() {
    let set = LinkedHashSet::<u32>::new();

    assert_tokens(&set, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);
}

#[test]
fn set_serde_tokens() {
    let mut set = LinkedHashSet::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);

    assert_tokens(
        &set,
        &[
            Token::Seq { len: Some(3) },
            Token::I32(10),
            Token::I32(20),
            Token::I32(30),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn set_serde_tokens_generic() {
    let mut set = LinkedHashSet::<char, BuildHasherDefault<FxHasher>>::default();
    set.insert('a');
    set.insert('b');
    set.insert('c');

    assert_tokens(
        &set,
        &[
            Token::Seq { len: Some(3) },
            Token::Char('a'),
            Token::Char('b'),
            Token::Char('c'),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn lru_serde_tokens_empty() {
    let map = LruCache::<char, u32>::new(16);

    assert_tokens(
        &map,
        &[
            Token::Struct {
                name: "LruCache",
                len: 2,
            },
            Token::Str("map"),
            Token::Map { len: Some(0) },
            Token::MapEnd,
            Token::Str("max_size"),
            token_usize(16),
            Token::StructEnd,
        ],
    );
}

#[test]
fn lru_serde_tokens() {
    let mut map = LruCache::new(16);
    map.insert('a', 10);
    map.insert('b', 20);
    map.insert('c', 30);

    assert_tokens(
        &map,
        &[
            Token::Struct {
                name: "LruCache",
                len: 2,
            },
            Token::Str("map"),
            Token::Map { len: Some(3) },
            Token::Char('a'),
            Token::I32(10),
            Token::Char('b'),
            Token::I32(20),
            Token::Char('c'),
            Token::I32(30),
            Token::MapEnd,
            Token::Str("max_size"),
            token_usize(16),
            Token::StructEnd,
        ],
    );
}

#[test]
fn lru_serde_tokens_empty_generic() {
    let map = LruCache::<char, u32, BuildHasherDefault<FxHasher>>::with_hasher(
        16,
        BuildHasherDefault::<FxHasher>::default(),
    );

    assert_tokens(
        &map,
        &[
            Token::Struct {
                name: "LruCache",
                len: 2,
            },
            Token::Str("map"),
            Token::Map { len: Some(0) },
            Token::MapEnd,
            Token::Str("max_size"),
            token_usize(16),
            Token::StructEnd,
        ],
    );
}

#[test]
fn lru_serde_tokens_generic() {
    let mut map = LruCache::with_hasher(16, BuildHasherDefault::<FxHasher>::default());
    map.insert('a', 10);
    map.insert('b', 20);
    map.insert('c', 30);

    assert_tokens(
        &map,
        &[
            Token::Struct {
                name: "LruCache",
                len: 2,
            },
            Token::Str("map"),
            Token::Map { len: Some(3) },
            Token::Char('a'),
            Token::I32(10),
            Token::Char('b'),
            Token::I32(20),
            Token::Char('c'),
            Token::I32(30),
            Token::MapEnd,
            Token::Str("max_size"),
            token_usize(16),
            Token::StructEnd,
        ],
    );
}
