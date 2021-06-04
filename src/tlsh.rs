use crate::helper::{b_mapping, find_quartiles, l_capturing, BUCKET_SIZE, WINDOW_SIZE};

/// A struct containing all required information from an input stream to generate a hash value.
///
/// An instance of this struct can be obtained by calling the function [`TlshBuilder::build`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tlsh {
    ver: Version,
    checksum: Vec<u8>,
    len: usize,
    q1ratio: u32,
    q2ratio: u32,
    codes: Vec<u8>,
}

impl Tlsh {
    /// Computes and returns the hash value in hex-encoded string format.
    pub fn hash(&self) -> String {
        let cap = self.ver.ver().len() + self.codes.len() * 2 + self.checksum.len() * 2 + 4;
        let mut result = String::with_capacity(cap);
        result.push_str(self.ver.ver());

        for ii in 0..self.checksum.len() {
            result.push_str(
                &format!("{:02X}", self.checksum[ii])
                    .chars()
                    .rev()
                    .collect::<String>(),
            );
        }
        result.push_str(
            &format!("{:02X}", self.len as u32)
                .chars()
                .rev()
                .collect::<String>(),
        );
        result.push_str(&format!("{:02X}", self.q1ratio << 4 | self.q2ratio));

        let len = self.codes.len();
        for ii in 0..len {
            result.push_str(&format!("{:02X}", self.codes[len - 1 - ii]));
        }

        result
    }
}

/// A builder struct for processing input stream(s).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TlshBuilder {
    buckets: [u32; BUCKET_SIZE],
    bucket_count: usize,
    checksum: u8,
    checksum_array: Vec<u8>,
    checksum_len: usize,
    code_size: usize,
    data_len: usize,
    slide_window: [u8; WINDOW_SIZE],
    ver: Version,
}

impl TlshBuilder {
    /// Constructs a new builder based on the number of buckets, checksum length and version.
    pub fn new(bucket: BucketKind, checksum: ChecksumKind, ver: Version) -> Self {
        let bucket_count = bucket.bucket_count();
        let checksum_len = checksum.checksum_len();

        Self {
            buckets: [0; BUCKET_SIZE],
            bucket_count,
            checksum: 0,
            checksum_array: vec![0; checksum_len],
            checksum_len,
            code_size: bucket_count >> 2,
            data_len: 0,
            slide_window: [0; WINDOW_SIZE],
            ver,
        }
    }

    /// Computes the quartiles and constructs the digest message and returns an instance of [`Tlsh`]
    /// that has all information needed to generate a hash value.
    pub fn build(&self) -> Tlsh {
        let (q1, q2, q3) = find_quartiles(&self.buckets, self.bucket_count);

        if q3 == 0 {
            panic!("q3 = 0")
        }

        let mut tmp = vec![0; self.code_size];
        for ii in 0..self.code_size {
            let mut h = 0;

            for jj in 0..4 {
                // Out of bound check?
                let kk = self.buckets[4 * ii + jj];
                if q3 < kk {
                    h += 3 << (jj * 2);
                } else if q2 < kk {
                    h += 2 << (jj * 2);
                } else if q1 < kk {
                    h += 1 << (jj * 2);
                }
            }

            tmp[ii] = h;
        }

        let len = l_capturing(self.data_len).unwrap();
        let q1ratio = (((q1 as f64 * 100.) / (q3 as f64)) as u32) % 16;
        let q2ratio = (((q2 as f64 * 100.) / (q3 as f64)) as u32) % 16;

        let checksum = if self.checksum_len == 1 {
            vec![self.checksum]
        } else {
            self.checksum_array.clone()
        };

        Tlsh {
            ver: self.ver,
            checksum: checksum,
            len,
            q1ratio,
            q2ratio,
            codes: tmp,
        }
    }

    /// Processes an input stream.
    pub fn update(&mut self, data: &[u8]) {
        self.update_from(data, 0, data.len());
    }

    /// Reads an input stream from an offset an processes it.
    ///
    /// # Parameters
    /// * data: input data to be added
    /// * offset: index in array from which data will be read
    /// * len: number of bytes to be read
    pub fn update_from(&mut self, data: &[u8], offset: usize, len: usize) {
        let mut j0 = self.data_len % WINDOW_SIZE;
        let (mut j1, mut j2, mut j3, mut j4) = (
            (j0 + WINDOW_SIZE - 1) % WINDOW_SIZE,
            (j0 + WINDOW_SIZE - 2) % WINDOW_SIZE,
            (j0 + WINDOW_SIZE - 3) % WINDOW_SIZE,
            (j0 + WINDOW_SIZE - 4) % WINDOW_SIZE,
        );

        let mut fed_len = self.data_len;

        for ii in offset..(offset + len) {
            self.slide_window[j0] = data[ii];

            if fed_len >= 4 {
                self.checksum = b_mapping(
                    0,
                    self.slide_window[j0],
                    self.slide_window[j1],
                    self.checksum,
                );

                if self.checksum_len > 1 {
                    self.checksum_array[0] = self.checksum;

                    for kk in 1..self.checksum_len {
                        self.checksum_array[kk] = b_mapping(
                            self.checksum_array[kk - 1],
                            self.slide_window[j0],
                            self.slide_window[j1],
                            self.checksum_array[kk],
                        )
                    }
                }

                // Select 6 triplets out of 10. The last four are processed in the next iteration.
                // A   - B   - C   - D   - E
                // j0   j_1   j2   j3   j4

                let mut r = b_mapping(
                    2,
                    self.slide_window[j0],
                    self.slide_window[j1],
                    self.slide_window[j2],
                );
                self.buckets[r as usize] += 1;

                r = b_mapping(
                    3,
                    self.slide_window[j0],
                    self.slide_window[j1],
                    self.slide_window[j3],
                );
                self.buckets[r as usize] += 1;

                r = b_mapping(
                    5,
                    self.slide_window[j0],
                    self.slide_window[j2],
                    self.slide_window[j3],
                );
                self.buckets[r as usize] += 1;

                r = b_mapping(
                    7,
                    self.slide_window[j0],
                    self.slide_window[j2],
                    self.slide_window[j4],
                );
                self.buckets[r as usize] += 1;

                r = b_mapping(
                    11,
                    self.slide_window[j0],
                    self.slide_window[j1],
                    self.slide_window[j4],
                );
                self.buckets[r as usize] += 1;

                r = b_mapping(
                    13,
                    self.slide_window[j0],
                    self.slide_window[j3],
                    self.slide_window[j4],
                );
                self.buckets[r as usize] += 1;
            }

            fed_len += 1;

            let tmp = j4;
            j4 = j3;
            j3 = j2;
            j2 = j1;
            j1 = j0;
            j0 = tmp;
        }

        self.data_len += len;
    }

    /// Clears the state of a builder, removing all data.
    pub fn reset(&mut self) {
        self.buckets.fill(0);
        self.checksum = 0;
        self.data_len = 0;
        self.slide_window.fill(0);
    }
}

/// An enum determining the number of buckets for hashing
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum BucketKind {
    ///
    Bucket128,
    ///
    Bucket256,
}

impl BucketKind {
    /// Returns the number of buckets.
    pub fn bucket_count(&self) -> usize {
        match self {
            BucketKind::Bucket128 => 128,
            BucketKind::Bucket256 => 256,
        }
    }
}

/// An enum determining the length of checksum.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ChecksumKind {
    /// TLSH uses one byte for checksum. The collision rate is 1/24.
    OneByte,
    /// TLSH uses three bytes for checksum. The collision rate is 1/5800.
    ThreeByte,
}

impl ChecksumKind {
    pub fn checksum_len(&self) -> usize {
        match self {
            ChecksumKind::OneByte => 1,
            ChecksumKind::ThreeByte => 3,
        }
    }
}

/// An enum representing the version of TLSH
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Version {
    /// Original version, mapping to an empty string ```""```.
    Original,
    /// Current version, mapping to an string ```"T1"```.
    Version4,
}

impl Version {
    pub fn ver(&self) -> &str {
        match self {
            Version::Original => "",
            Version::Version4 => "T1",
        }
    }
}
