use breaking_attr::breaking;

#[cfg(test)]
#[test]
fn all_hashers() {
    #![allow(unused)]

    #[breaking("oryQa81ib3SsqJDTSVv02R_SrNGRhnidqjYh8EOs0es=")]
    pub const DEFAULT: &str = "The default hasher";

    #[breaking(blake3 = "gp9sbaqqzm3uo1HPBf_lly1pSzbAOQN-1ak8YgtjFBI=")]
    pub const BLAKE3: &str = "blake3";

    #[breaking(sha2 = "vBiL_LXe0fzq8CYZidX08ua7mMMB7T_KmbWqS9w6ZHM=")]
    pub const SHA2: &str = "sha2";

    #[breaking(sha256 = "yFGGvXNrkGi9W6EkurKoaR-YVX-wkVrvd3OLflrGyG0=")]
    pub const SHA256: &str = "sha256";

    #[breaking(sha224 = "eDkZAHeOIImW3JGjkwEiOzCEy-JIeXgkAxE0gg==")]
    pub const SHA224: &str = "sha224";

    #[breaking(sha384 = "dUZc1MuQAoTLrSKVuK1ovEIdz_orTO4QbGvUWOevtQ8SwyqlM-8_ppWwoUzOaC82")]
    pub const SHA384: &str = "sha384";

    #[breaking(
        sha512 = "q6pVPUidwbMP6dIZ7AkRtX1lI-BQ41T3B5xYl1d2-AuEyAgPcf3OrAukc5F4ipRbdNoHDmHJrh5pX0-Ax-RQFg=="
    )]
    pub const SHA512: &str = "sha512";

    #[breaking(sha512_224 = "mnnxQ56r1rLGV66BDYBdwkP_No-pI0-GQfruQw==")]
    pub const SHA512_224: &str = "sha512_224";

    #[breaking(sha512_256 = "Io7SHXF6BTzGjmPifLa58nSaz8bxdxXKoufjhoTPcQw=")]
    pub const SHA512_256: &str = "sha512_256";

    #[breaking(md5 = "63_HcFESCMbwzC0-dDXfXA==")]
    pub const MD5: &str = "md5";

    {
        #[breaking("aaB9pcdXQFFioZCKXy0aOEg7ZuXjek5CUCzh02RHqwA=")]
        // This comment should not appear in the TokenStream
        pub const COMMENTED: &str = "This might have a comment that shouldn't affect the hash";
    }
    {
        // This hash should match the one above despite the lack of comment
        #[breaking("aaB9pcdXQFFioZCKXy0aOEg7ZuXjek5CUCzh02RHqwA=")]
        pub const COMMENTED: &str = "This might have a comment that shouldn't affect the hash";
    }

    {
        #[breaking("6cZh6KDiaSfJeoXW5u-P9aH4W80nb6JK3WnouH8Ym58=")]
        /// This doc comment should appear in the TokenStream
        pub const DOC_COMMENTED: &str = "This has a doc comment that SHOULD affect the hash.";
    }
    {
        #[breaking("spnWzg4vbjFvxbA0317-tPwQ_vgC-1TecAROamFt4Ls=")]
        /// This is a different doc comment which should make the hash different.
        pub const DOC_COMMENTED: &str = "This has a doc comment that SHOULD affect the hash.";
    }

    {
        #[breaking("t57Llq4DaYqzAXp36UpbUTbflJz1QajYTOEtLULe5hA=")]
        pub const MULTILINE_STRING: &str = "This is a
multiline string";
    };
    {
        // This hash should NOT be the same as above. Newlines in strings are significant.
        #[breaking("AyMjo9dgtTb1Tz5kXcwGwyp6nLPCUo5SC0djYBlJ-1c=")]
        pub const MULTILINE_STRING: &str = "This is a
multiline
string";
    };

    {
        #[breaking("00RJnsBfjXxZrjOCz1NQV6iceODn8Wm8ZpoG4H8QzEk=")]
        pub const RAW_STRING: &str = r#"This is a raw string"#;
        #[breaking("qRYqMXh02j2-x-YNBFTjdgWkyaoaLLhIw8T58ICtDTc=")]
        pub const DOUBLE_HASH_RAW_STRING: &str = r##"This is a raw string with a " in it"##;
    }
    {
        // These hashes should be different
        #[breaking("W75LdIPgEspahaQq296qHWe00HwewlFjKrT463guMVQ=")]
        pub const RAW_STRING: &str = r#"This is a different raw string"#;
        #[breaking("dI7G_TPPFLE36HUzDMR_OCsuhosi69CEz0HKglfxcjg=")]
        pub const DOUBLE_HASH_RAW_STRING: &str =
            r##"This is a different raw string with a " in it"##;
    }
}
