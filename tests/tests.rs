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

    // todo: other sha hashers

    #[breaking(md5 = "63_HcFESCMbwzC0-dDXfXA==")]
    pub const MD5: &str = "md5";
}
