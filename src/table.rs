use core::panic;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::util::grow_capacity;
use crate::value::Value;

// ################################################################
// Table
// ################################################################

const TABLE_MAX_LOAD: f32 = 0.75;

#[derive(Debug, Copy, Clone)]
enum TableError {
    InsertKeyError,
    AddTableError,
}

impl Display for TableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            TableError::InsertKeyError => { "cannot insert key into table." }
            TableError::AddTableError => { "cannot add table to other table." }
        };

        write!(f, "table error: {}", message)
    }
}

#[derive(Default)]
struct Table {
    count: usize,
    capacity: usize,
    entries: Vec<Option<Entry>>,
}

impl Table {
    fn new() -> Self {
        Table {
            count: 0,
            capacity: 1,
            entries: Vec::default(),
        }
    }

    /// Returns a value given a key.
    fn get(&mut self, key: &str) -> Option<&Value> {
        if self.count == 0 {
            return None;
        }

        let entry =
            Table::find_entry(&mut self.entries, &key, self.capacity);

        return match entry {
            Some(i) => {
                match &self.entries.get(i).unwrap() {
                    Some(e) => {
                        Some(&e.value)
                     },
                    None => { None },
                }
            }
            None => { None }
        }
    }
    
    fn get_unchecked(&mut self, key: &str) -> &Value {
        if self.count == 0 {
            panic!("{key} not found.");
        }

        let entry =
            Table::find_entry(&mut self.entries, key, self.capacity);

        return match entry {
            Some(i) => {
                &self.entries.get(i).unwrap().as_ref().unwrap().value
            }
            None => { panic!("{key} not found.") }
        }
    }

    fn delete(&mut self, key: &str) -> bool {
        if self.count == 0 {
            return false;
        }

        // Find the entry.
        let find =
            Table::find_entry(&mut self.entries, key, self.capacity);
        return match find {
            // Place a tombstone in the entry.
            Some(i) => {
                let entry = self.entries.get_mut(i).unwrap().as_mut().unwrap();
                entry.is_tombstone = true;
                true
            }
            None => { false }
        };
    }

    /// Inserts an entity into the hash map. Resizes
    /// the table if the capacity has been reached.
    fn insert(&mut self, key: impl ToString, value: Value) -> Result<(), Box<TableError>> {
        let key = key.to_string();
        // Grow the capacity if the capacity has been reached.
        if (self.count + 1) as f32 > (self.capacity as f32) * TABLE_MAX_LOAD {
            let new_capacity = grow_capacity(self.capacity);
            self.adjust_capacity(new_capacity);
        }

        // Check if the entry is already in the hash map
        // Should always find a spot in the hashmap to insert the new elements. 
        let bucket_index = Table::find_entry(&mut self.entries, &key, self.capacity);
        let entry = Entry::new(key, value);
        *self.entries.get_mut(bucket_index.unwrap()).unwrap() = Some(entry);
        self.count += 1;
        Ok(())
    }

    /// Finds the first occurrence of the key or the first empty bucket in the hash table with
    /// linear probing. 
    fn find_entry(map: &mut Vec<Option<Entry>>, key: &str , capacity: usize) -> Option<usize> {
        let hash = fnv1a(key.as_bytes());
        let mut i = index(hash, capacity);
        let start_i = i;
        loop {
            let entry = &map[i];
            match entry {
                // The hash table DOES NOT contain the entry
                None => { return Some(i) }
                // The hash table DOES contain the entry
                Some(e) => {
                    if e.key == *key && !e.is_tombstone  {
                        return Some(i);
                    }
                }
            }

            i = (i + 1) % capacity;

            if start_i == i {
                return None;
            }

        }
    }

    /// Creates a new vector that is twice the length of the previous. Takes every Option<Entry> in
    /// the old vector and rebuilds the new HashTable from scratch by re-inserting every entry into the
    /// new empty array.
    fn adjust_capacity(&mut self, new_capacity: usize) {
        // Create an 'array' of Vec<Option<Entry>> of the None enum with the new capacity.
        let mut new: Vec<Option<Entry>> =
            (0..new_capacity).map(|_| None).collect();

        // Reset capacity.
        self.count = 0;

        for mut old_entry in self.entries.iter_mut() {
            match old_entry {
                Some(e) => {
                    // Only add the entries which are not tombstones.
                    if !e.is_tombstone {
                        // Will always return usize because the array was just initialized.
                        let destination = Table::find_entry(&mut new, &e.key, self.capacity);
                            *new.get_mut(destination.unwrap()).unwrap() = old_entry.take();
                            self.count = self.count + 1;
                    }
                }
                None => { continue }
            }
        }

        self.entries = new;
        self.capacity = new_capacity;

    }

    /// Adds every entry from table A to table B, returns an error if conflicts are present.
    fn add_all(from: &mut Table, to: &mut Table) -> Result<(), TableError> {
        for entry in from.entries.iter_mut() {
            match entry {
                None => {}
                Some(_) => {
                    let take  = entry.take().unwrap();
                    let result = to.insert(take.key, take.value);
                    match result {
                        Ok(_) => {}
                        Err(_) => { return Err(TableError::AddTableError)}
                    }
                }
            }
        }
        Ok(())
    }
}

struct Entry {
    key: String,
    value: Value,
    hash: u64,
    /// Sentinel entry to record when an entry has been deleted.
    is_tombstone: bool,
}

impl Entry {
    pub fn new(key: String, value: Value) -> Self {
        let hash = fnv1a(key.as_bytes());

        Entry {
            key,
            value,
            hash,
            is_tombstone: false,
        }
    }
}

// ###########################################################
// Utility functions
// ###########################################################

/// Uses the FNV-1a hash to return a fixed sized u32 regardless of the input's size
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;

    for byte in bytes.iter() {
        hash = hash ^ (*byte as u64);
        hash = hash.wrapping_mul(0x100000001b3);
    }

    return hash
}

fn index(hash: u64, capacity: usize) -> usize {
    (hash % capacity as u64 ) as usize
}

mod tests {
    use crate::table::{fnv1a, Table, TableError};
    use crate::value::{allocate_object, Value};

    #[test]
    fn table_test_insert() -> Result<(), Box<TableError>> {
        let mut table = Table::default();
        table.insert("yellow", allocate_object("stone"))?;
        Ok(())
    }

    #[test]
    fn table_test_get() {
        let mut table = Table::default();
        let _ = table.insert("yellow", allocate_object("stone"));
        let _ = table.insert("bicycle", allocate_object("patagonia"));

        assert_eq!(table.get_unchecked("yellow"), "stone");
        assert_eq!(table.get_unchecked("bicycle"), "patagonia");
    }

    #[test]
    fn table_test_delete() {
        let mut table = Table::default();
        let _ = table.insert("yellow", allocate_object("stone"));
        let _ = table.insert("bicycle", allocate_object("patagonia"));

        assert_eq!(table.get_unchecked("yellow"), "stone");
        assert_eq!(table.get_unchecked("bicycle"), "patagonia");

        table.delete("yellow");
        table.delete("bicycle");
        assert_eq!(None, table.get("yellow"));
        assert_eq!(None, table.get("bicycle"))
    }

    #[test]
    fn table_test_integration() {
        let mut table = Table::default();
        let _ = table.insert("yellow", allocate_object("stone"));
        let _ = table.insert("bicycle", allocate_object("patagonia"));
        let _ = table.insert("van", allocate_object("life"));

        assert_eq!(table.get_unchecked("yellow"), "stone");
        assert_eq!(table.get_unchecked("bicycle"), "patagonia");
        assert_eq!(table.get_unchecked("van"), "life");

        table.delete("yellow");
        table.delete("bicycle");

        assert_eq!(None, table.get("yellow"));
        assert_eq!(None, table.get("bicycle"));
        assert_eq!(table.get_unchecked("van"), "life")
    }

    #[test]
    fn table_test_values() {
        let mut table = Table::default();
        let _ = table.insert("answer", Value::number_value(42.));
        let _ = table.insert("happy?", Value::bool_val(true));
        let _ = table.insert("name", allocate_object("kristoff"));
        let _ = table.insert("null", Value::nil_value());

        assert_eq!(table.get_unchecked("answer"), &42.);
        assert_eq!(table.get_unchecked("happy?"), &true);
        assert_eq!(table.get_unchecked("name"), "kristoff");
        assert_eq!(table.get_unchecked("null"), &0.);
    }

    #[test]
    fn hash_tests() {
        assert_eq!(fnv1a(b""), 0xcbf29ce484222325);
        assert_eq!(fnv1a(b"a"), 0xaf63dc4c8601ec8c);
        assert_eq!(fnv1a(b"b"), 0xaf63df4c8601f1a5);
        assert_eq!(fnv1a(b"c"), 0xaf63de4c8601eff2);
        assert_eq!(fnv1a(b"d"), 0xaf63d94c8601e773);
        assert_eq!(fnv1a(b"e"), 0xaf63d84c8601e5c0);
        assert_eq!(fnv1a(b"f"), 0xaf63db4c8601ead9);
        assert_eq!(fnv1a(b"fo"), 0x08985907b541d342);
        assert_eq!(fnv1a(b"foo"), 0xdcb27518fed9d577);
        assert_eq!(fnv1a(b"foob"), 0xdd120e790c2512af);
        assert_eq!(fnv1a(b"fooba"), 0xcac165afa2fef40a);
        assert_eq!(fnv1a(b"foobar"), 0x85944171f73967e8);
        assert_eq!(fnv1a(b"\0"), 0xaf63bd4c8601b7df);
        assert_eq!(fnv1a(b"a\0"), 0x089be207b544f1e4);
        assert_eq!(fnv1a(b"b\0"), 0x08a61407b54d9b5f);
        assert_eq!(fnv1a(b"c\0"), 0x08a2ae07b54ab836);
        assert_eq!(fnv1a(b"d\0"), 0x0891b007b53c4869);
        assert_eq!(fnv1a(b"e\0"), 0x088e4a07b5396540);
        assert_eq!(fnv1a(b"f\0"), 0x08987c07b5420ebb);
        assert_eq!(fnv1a(b"fo\0"), 0xdcb28a18fed9f926);
        assert_eq!(fnv1a(b"foo\0"), 0xdd1270790c25b935);
        assert_eq!(fnv1a(b"foob\0"), 0xcac146afa2febf5d);
        assert_eq!(fnv1a(b"fooba\0"), 0x8593d371f738acfe);
        assert_eq!(fnv1a(b"foobar\0"), 0x34531ca7168b8f38);
        assert_eq!(fnv1a(b"ch"), 0x08a25607b54a22ae);
        assert_eq!(fnv1a(b"cho"), 0xf5faf0190cf90df3);
        assert_eq!(fnv1a(b"chon"), 0xf27397910b3221c7);
        assert_eq!(fnv1a(b"chong"), 0x2c8c2b76062f22e0);
        assert_eq!(fnv1a(b"chongo"), 0xe150688c8217b8fd);
        assert_eq!(fnv1a(b"chongo "), 0xf35a83c10e4f1f87);
        assert_eq!(fnv1a(b"chongo w"), 0xd1edd10b507344d0);
        assert_eq!(fnv1a(b"chongo wa"), 0x2a5ee739b3ddb8c3);
        assert_eq!(fnv1a(b"chongo was"), 0xdcfb970ca1c0d310);
        assert_eq!(fnv1a(b"chongo was "), 0x4054da76daa6da90);
        assert_eq!(fnv1a(b"chongo was h"), 0xf70a2ff589861368);
        assert_eq!(fnv1a(b"chongo was he"), 0x4c628b38aed25f17);
        assert_eq!(fnv1a(b"chongo was her"), 0x9dd1f6510f78189f);
        assert_eq!(fnv1a(b"chongo was here"), 0xa3de85bd491270ce);
        assert_eq!(fnv1a(b"chongo was here!"), 0x858e2fa32a55e61d);
        assert_eq!(fnv1a(b"chongo was here!\n"), 0x46810940eff5f915);
        assert_eq!(fnv1a(b"ch\0"), 0xf5fadd190cf8edaa);
        assert_eq!(fnv1a(b"cho\0"), 0xf273ed910b32b3e9);
        assert_eq!(fnv1a(b"chon\0"), 0x2c8c5276062f6525);
        assert_eq!(fnv1a(b"chong\0"), 0xe150b98c821842a0);
        assert_eq!(fnv1a(b"chongo\0"), 0xf35aa3c10e4f55e7);
        assert_eq!(fnv1a(b"chongo \0"), 0xd1ed680b50729265);
        assert_eq!(fnv1a(b"chongo w\0"), 0x2a5f0639b3dded70);
        assert_eq!(fnv1a(b"chongo wa\0"), 0xdcfbaa0ca1c0f359);
        assert_eq!(fnv1a(b"chongo was\0"), 0x4054ba76daa6a430);
        assert_eq!(fnv1a(b"chongo was \0"), 0xf709c7f5898562b0);
        assert_eq!(fnv1a(b"chongo was h\0"), 0x4c62e638aed2f9b8);
        assert_eq!(fnv1a(b"chongo was he\0"), 0x9dd1a8510f779415);
        assert_eq!(fnv1a(b"chongo was her\0"), 0xa3de2abd4911d62d);
        assert_eq!(fnv1a(b"chongo was here\0"), 0x858e0ea32a55ae0a);
        assert_eq!(fnv1a(b"chongo was here!\0"), 0x46810f40eff60347);
        assert_eq!(fnv1a(b"chongo was here!\n\0"), 0xc33bce57bef63eaf);
        assert_eq!(fnv1a(b"cu"), 0x08a24307b54a0265);
        assert_eq!(fnv1a(b"cur"), 0xf5b9fd190cc18d15);
        assert_eq!(fnv1a(b"curd"), 0x4c968290ace35703);
        assert_eq!(fnv1a(b"curds"), 0x07174bd5c64d9350);
        assert_eq!(fnv1a(b"curds "), 0x5a294c3ff5d18750);
        assert_eq!(fnv1a(b"curds a"), 0x05b3c1aeb308b843);
        assert_eq!(fnv1a(b"curds an"), 0xb92a48da37d0f477);
        assert_eq!(fnv1a(b"curds and"), 0x73cdddccd80ebc49);
        assert_eq!(fnv1a(b"curds and "), 0xd58c4c13210a266b);
        assert_eq!(fnv1a(b"curds and w"), 0xe78b6081243ec194);
        assert_eq!(fnv1a(b"curds and wh"), 0xb096f77096a39f34);
        assert_eq!(fnv1a(b"curds and whe"), 0xb425c54ff807b6a3);
        assert_eq!(fnv1a(b"curds and whey"), 0x23e520e2751bb46e);
        assert_eq!(fnv1a(b"curds and whey\n"), 0x1a0b44ccfe1385ec);
        assert_eq!(fnv1a(b"cu\0"), 0xf5ba4b190cc2119f);
        assert_eq!(fnv1a(b"cur\0"), 0x4c962690ace2baaf);
        assert_eq!(fnv1a(b"curd\0"), 0x0716ded5c64cda19);
        assert_eq!(fnv1a(b"curds\0"), 0x5a292c3ff5d150f0);
        assert_eq!(fnv1a(b"curds \0"), 0x05b3e0aeb308ecf0);
        assert_eq!(fnv1a(b"curds a\0"), 0xb92a5eda37d119d9);
        assert_eq!(fnv1a(b"curds an\0"), 0x73ce41ccd80f6635);
        assert_eq!(fnv1a(b"curds and\0"), 0xd58c2c132109f00b);
        assert_eq!(fnv1a(b"curds and \0"), 0xe78baf81243f47d1);
        assert_eq!(fnv1a(b"curds and w\0"), 0xb0968f7096a2ee7c);
        assert_eq!(fnv1a(b"curds and wh\0"), 0xb425a84ff807855c);
        assert_eq!(fnv1a(b"curds and whe\0"), 0x23e4e9e2751b56f9);
        assert_eq!(fnv1a(b"curds and whey\0"), 0x1a0b4eccfe1396ea);
        assert_eq!(fnv1a(b"curds and whey\n\0"), 0x54abd453bb2c9004);
        assert_eq!(fnv1a(b"hi"), 0x08ba5f07b55ec3da);
        assert_eq!(fnv1a(b"hi\0"), 0x337354193006cb6e);
        assert_eq!(fnv1a(b"hello"), 0xa430d84680aabd0b);
        assert_eq!(fnv1a(b"hello\0"), 0xa9bc8acca21f39b1);
        assert_eq!(fnv1a(b"\xff\x00\x00\x01"), 0x6961196491cc682d);
        assert_eq!(fnv1a(b"\x01\x00\x00\xff"), 0xad2bb1774799dfe9);
        assert_eq!(fnv1a(b"\xff\x00\x00\x02"), 0x6961166491cc6314);
        assert_eq!(fnv1a(b"\x02\x00\x00\xff"), 0x8d1bb3904a3b1236);
        assert_eq!(fnv1a(b"\xff\x00\x00\x03"), 0x6961176491cc64c7);
        assert_eq!(fnv1a(b"\x03\x00\x00\xff"), 0xed205d87f40434c7);
        assert_eq!(fnv1a(b"\xff\x00\x00\x04"), 0x6961146491cc5fae);
        assert_eq!(fnv1a(b"\x04\x00\x00\xff"), 0xcd3baf5e44f8ad9c);
        assert_eq!(fnv1a(b"\x40\x51\x4e\x44"), 0xe3b36596127cd6d8);
        assert_eq!(fnv1a(b"\x44\x4e\x51\x40"), 0xf77f1072c8e8a646);
        assert_eq!(fnv1a(b"\x40\x51\x4e\x4a"), 0xe3b36396127cd372);
        assert_eq!(fnv1a(b"\x4a\x4e\x51\x40"), 0x6067dce9932ad458);
        assert_eq!(fnv1a(b"\x40\x51\x4e\x54"), 0xe3b37596127cf208);
        assert_eq!(fnv1a(b"\x54\x4e\x51\x40"), 0x4b7b10fa9fe83936);
        assert_eq!(fnv1a(b"127.0.0.1"), 0xaabafe7104d914be);
        assert_eq!(fnv1a(b"127.0.0.1\0"), 0xf4d3180b3cde3eda);
        assert_eq!(fnv1a(b"127.0.0.2"), 0xaabafd7104d9130b);
        assert_eq!(fnv1a(b"127.0.0.2\0"), 0xf4cfb20b3cdb5bb1);
        assert_eq!(fnv1a(b"127.0.0.3"), 0xaabafc7104d91158);
        assert_eq!(fnv1a(b"127.0.0.3\0"), 0xf4cc4c0b3cd87888);
        assert_eq!(fnv1a(b"64.81.78.68"), 0xe729bac5d2a8d3a7);
        assert_eq!(fnv1a(b"64.81.78.68\0"), 0x74bc0524f4dfa4c5);
        assert_eq!(fnv1a(b"64.81.78.74"), 0xe72630c5d2a5b352);
        assert_eq!(fnv1a(b"64.81.78.74\0"), 0x6b983224ef8fb456);
        assert_eq!(fnv1a(b"64.81.78.84"), 0xe73042c5d2ae266d);
        assert_eq!(fnv1a(b"64.81.78.84\0"), 0x8527e324fdeb4b37);
        assert_eq!(fnv1a(b"feedface"), 0x0a83c86fee952abc);
        assert_eq!(fnv1a(b"feedface\0"), 0x7318523267779d74);
        assert_eq!(fnv1a(b"feedfacedaffdeed"), 0x3e66d3d56b8caca1);
        assert_eq!(fnv1a(b"feedfacedaffdeed\0"), 0x956694a5c0095593);
        assert_eq!(fnv1a(b"feedfacedeadbeef"), 0xcac54572bb1a6fc8);
        assert_eq!(fnv1a(b"feedfacedeadbeef\0"), 0xa7a4c9f3edebf0d8);
        assert_eq!(fnv1a(b"line 1\nline 2\nline 3"), 0x7829851fac17b143);
        assert_eq!(fnv1a(b"chongo <Landon Curt Noll> /\\../\\"), 0x2c8f4c9af81bcf06);
        assert_eq!(fnv1a(b"chongo <Landon Curt Noll> /\\../\\\0"), 0xd34e31539740c732);
        assert_eq!(fnv1a(b"chongo (Landon Curt Noll) /\\../\\"), 0x3605a2ac253d2db1);
        assert_eq!(fnv1a(b"chongo (Landon Curt Noll) /\\../\\\0"), 0x08c11b8346f4a3c3);
        assert_eq!(fnv1a(b"http://antwrp.gsfc.nasa.gov/apod/astropix.html"), 0x6be396289ce8a6da);
        assert_eq!(fnv1a(b"http://en.wikipedia.org/wiki/Fowler_Noll_Vo_hash"), 0xd9b957fb7fe794c5);
        assert_eq!(fnv1a(b"http://epod.usra.edu/"), 0x05be33da04560a93);
        assert_eq!(fnv1a(b"http://exoplanet.eu/"), 0x0957f1577ba9747c);
        assert_eq!(fnv1a(b"http://hvo.wr.usgs.gov/cam3/"), 0xda2cc3acc24fba57);
        assert_eq!(fnv1a(b"http://hvo.wr.usgs.gov/cams/HMcam/"), 0x74136f185b29e7f0);
        assert_eq!(fnv1a(b"http://hvo.wr.usgs.gov/kilauea/update/deformation.html"), 0xb2f2b4590edb93b2);
        assert_eq!(fnv1a(b"http://hvo.wr.usgs.gov/kilauea/update/images.html"), 0xb3608fce8b86ae04);
        assert_eq!(fnv1a(b"http://hvo.wr.usgs.gov/kilauea/update/maps.html"), 0x4a3a865079359063);
        assert_eq!(fnv1a(b"http://hvo.wr.usgs.gov/volcanowatch/current_issue.html"), 0x5b3a7ef496880a50);
        assert_eq!(fnv1a(b"http://neo.jpl.nasa.gov/risk/"), 0x48fae3163854c23b);
        assert_eq!(fnv1a(b"http://norvig.com/21-days.html"), 0x07aaa640476e0b9a);
        assert_eq!(fnv1a(b"http://primes.utm.edu/curios/home.php"), 0x2f653656383a687d);
        assert_eq!(fnv1a(b"http://slashdot.org/"), 0xa1031f8e7599d79c);
        assert_eq!(fnv1a(b"http://tux.wr.usgs.gov/Maps/155.25-19.5.html"), 0xa31908178ff92477);
        assert_eq!(fnv1a(b"http://volcano.wr.usgs.gov/kilaueastatus.php"), 0x097edf3c14c3fb83);
        assert_eq!(fnv1a(b"http://www.avo.alaska.edu/activity/Redoubt.php"), 0xb51ca83feaa0971b);
        assert_eq!(fnv1a(b"http://www.dilbert.com/fast/"), 0xdd3c0d96d784f2e9);
        assert_eq!(fnv1a(b"http://www.fourmilab.ch/gravitation/orbits/"), 0x86cd26a9ea767d78);
        assert_eq!(fnv1a(b"http://www.fpoa.net/"), 0xe6b215ff54a30c18);
        assert_eq!(fnv1a(b"http://www.ioccc.org/index.html"), 0xec5b06a1c5531093);
        assert_eq!(fnv1a(b"http://www.isthe.com/cgi-bin/number.cgi"), 0x45665a929f9ec5e5);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/bio.html"), 0x8c7609b4a9f10907);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/index.html"), 0x89aac3a491f0d729);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/src/calc/lucas-calc"), 0x32ce6b26e0f4a403);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/astro/venus2004.html"), 0x614ab44e02b53e01);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/astro/vita.html"), 0xfa6472eb6eef3290);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/comp/c/expert.html"), 0x9e5d75eb1948eb6a);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/comp/calc/index.html"), 0xb6d12ad4a8671852);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/comp/fnv/index.html"), 0x88826f56eba07af1);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/math/number/howhigh.html"), 0x44535bf2645bc0fd);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/math/number/number.html"), 0x169388ffc21e3728);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/math/prime/mersenne.html"), 0xf68aac9e396d8224);
        assert_eq!(fnv1a(b"http://www.isthe.com/chongo/tech/math/prime/mersenne.html#largest"), 0x8e87d7e7472b3883);
        assert_eq!(fnv1a(b"http://www.lavarnd.org/cgi-bin/corpspeak.cgi"), 0x295c26caa8b423de);
        assert_eq!(fnv1a(b"http://www.lavarnd.org/cgi-bin/haiku.cgi"), 0x322c814292e72176);
        assert_eq!(fnv1a(b"http://www.lavarnd.org/cgi-bin/rand-none.cgi"), 0x8a06550eb8af7268);
        assert_eq!(fnv1a(b"http://www.lavarnd.org/cgi-bin/randdist.cgi"), 0xef86d60e661bcf71);
        assert_eq!(fnv1a(b"http://www.lavarnd.org/index.html"), 0x9e5426c87f30ee54);
        assert_eq!(fnv1a(b"http://www.lavarnd.org/what/nist-test.html"), 0xf1ea8aa826fd047e);
        assert_eq!(fnv1a(b"http://www.macosxhints.com/"), 0x0babaf9a642cb769);
        assert_eq!(fnv1a(b"http://www.mellis.com/"), 0x4b3341d4068d012e);
        assert_eq!(fnv1a(b"http://www.nature.nps.gov/air/webcams/parks/havoso2alert/havoalert.cfm"), 0xd15605cbc30a335c);
        assert_eq!(fnv1a(b"http://www.nature.nps.gov/air/webcams/parks/havoso2alert/timelines_24.cfm"), 0x5b21060aed8412e5);
        assert_eq!(fnv1a(b"http://www.paulnoll.com/"), 0x45e2cda1ce6f4227);
        assert_eq!(fnv1a(b"http://www.pepysdiary.com/"), 0x50ae3745033ad7d4);
        assert_eq!(fnv1a(b"http://www.sciencenews.org/index/home/activity/view"), 0xaa4588ced46bf414);
        assert_eq!(fnv1a(b"http://www.skyandtelescope.com/"), 0xc1b0056c4a95467e);
        assert_eq!(fnv1a(b"http://www.sput.nl/~rob/sirius.html"), 0x56576a71de8b4089);
        assert_eq!(fnv1a(b"http://www.systemexperts.com/"), 0xbf20965fa6dc927e);
        assert_eq!(fnv1a(b"http://www.tq-international.com/phpBB3/index.php"), 0x569f8383c2040882);
        assert_eq!(fnv1a(b"http://www.travelquesttours.com/index.htm"), 0xe1e772fba08feca0);
        assert_eq!(fnv1a(b"http://www.wunderground.com/global/stations/89606.html"), 0x4ced94af97138ac4);
    }
}
