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
    
    #[breaking(sha512 = "q6pVPUidwbMP6dIZ7AkRtX1lI-BQ41T3B5xYl1d2-AuEyAgPcf3OrAukc5F4ipRbdNoHDmHJrh5pX0-Ax-RQFg==")]
    pub const SHA512: &str = "sha512";
    
    #[breaking(sha512_224 = "mnnxQ56r1rLGV66BDYBdwkP_No-pI0-GQfruQw==")]
    pub const SHA512_224: &str = "sha512_224";
    
    #[breaking(sha512_256 = "Io7SHXF6BTzGjmPifLa58nSaz8bxdxXKoufjhoTPcQw=")]
    pub const SHA512_256: &str = "sha512_256";
    
    #[breaking(md5 = "63_HcFESCMbwzC0-dDXfXA==")]
    pub const MD5: &str = "md5";
}
