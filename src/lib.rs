use num_bigint::{BigUint, RandBigInt};
use rand::Rng;

/// A struct representing the Zero-Knowledge Proof (ZKP) parameters.
///
/// This struct holds the parameters required for performing Zero-Knowledge Proofs,
/// including the prime numbers `p` and `q`, and the generators `alpha` and `beta`.
pub struct ZKP {
    pub p: BigUint,
    pub q: BigUint,
    pub alpha: BigUint,
    pub beta: BigUint,
}

impl ZKP {
    /// Creates a new `ZKP` instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `p` - A reference to a `BigUint` representing the prime number `p`.
    /// * `q` - A reference to a `BigUint` representing the prime number `q`.
    /// * `alpha` - A reference to a `BigUint` representing the generator `alpha`.
    /// * `beta` - A reference to a `BigUint` representing the generator `beta`.
    ///
    /// # Returns
    ///
    /// A new `ZKP` instance initialized with the provided parameters.
    pub fn new(p: &BigUint, q: &BigUint, alpha: &BigUint, beta: &BigUint) -> Self {
        Self {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        }
    }

    /// Computes a pair of values based on the ZKP parameters and an exponent.
    ///
    /// This method calculates the pair of outputs as `(alpha^exp mod p, beta^exp mod p)`.
    ///
    /// # Arguments
    ///
    /// * `exp` - A reference to a `BigUint` representing the exponent.
    ///
    /// # Returns
    ///
    /// A tuple containing the computed values as `BigUint`.
    pub fn compute_pair(&self, exp: &BigUint) -> (BigUint, BigUint) {
        let p1 = self.alpha.modpow(exp, &self.p);
        let p2 = self.beta.modpow(exp, &self.p);
        (p1, p2)
    }

    /// Solves for the value `s` based on the provided parameters.
    ///
    /// The solution is computed using the formula `s = k - c * x mod q`.
    ///
    /// # Arguments
    ///
    /// * `k` - A reference to a `BigUint` representing `k`.
    /// * `c` - A reference to a `BigUint` representing `c`.
    /// * `x` - A reference to a `BigUint` representing `x`.
    ///
    /// # Returns
    ///
    /// A `BigUint` representing the computed value `s`.
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c * x).modpow(&BigUint::from(1u32), &self.q);
        }
        &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q)
    }

    /// Verifies the conditions for the ZKP.
    ///
    /// The verification checks the conditions:
    /// 1. `r1 = alpha^s * y1^c`
    /// 2. `r2 = beta^s * y2^c`
    ///
    /// # Arguments
    ///
    /// * `r1` - A reference to a `BigUint` representing the first response.
    /// * `r2` - A reference to a `BigUint` representing the second response.
    /// * `y1` - A reference to a `BigUint` representing the first witness.
    /// * `y2` - A reference to a `BigUint` representing the second witness.
    /// * `c`  - A reference to a `BigUint` representing the challenge value.
    /// * `s`  - A reference to a `BigUint` representing the response value.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the verification conditions are met.
    pub fn verify(
        &self, r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, c: &BigUint, s: &BigUint,
    ) -> bool {
        let cond1 = *r1
            == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        let cond2 = *r2
            == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        cond1 && cond2
    }

    /// Generates a random number below the specified bound.
    ///
    /// This method uses a secure random number generator to produce a random BigUint
    /// that is less than the provided bound.
    ///
    /// # Arguments
    /// * `bound` - The upper bound for the random number generation.
    ///
    /// # Returns
    /// A `BigUint` representing the generated random number.
    pub fn generate_random_number_below(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_below(bound)
    }

    /// Generates a random alphanumeric string of the specified size.
    ///
    /// This method uses a secure random number generator to produce a random string containing
    /// alphanumeric characters.
    ///
    /// # Arguments
    /// * `size` - The length of the string to be generated.
    ///
    /// # Returns
    /// A `String` representing the generated random alphanumeric string.
    pub fn generate_random_string(size: usize) -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }

    /// Retrieves the ZKP constants used in the Zero-Knowledge Proof protocol.
    ///
    /// This method returns the constants `alpha`, `beta`, `p`, and `q` which are
    /// fundamental to the ZKP computations.
    ///
    /// # Returns
    ///
    /// A tuple containing the constants `(alpha, beta, p, q)` as `BigUint`.
    #[rustfmt::skip]
    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint)
    {
        let p = BigUint::from_bytes_be(&hex::decode(
            "B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B6160\
             73E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCB\
             DD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF\
             5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );

        let alpha = BigUint::from_bytes_be(
            &hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(),
        );

        // beta = alpha^i is also a generator
        let exp = BigUint::from_bytes_be(&hex::decode("266FEA1E5C41564B777E69").unwrap());
        let beta = alpha.modpow(&exp, &p);

        (alpha, beta, p, q)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_example() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        let zkp = ZKP::new(&p, &q, &alpha, &beta);

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c = BigUint::from(4u32);

        let (y1, y2) = zkp.compute_pair(&x);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let (r1, r2) = zkp.compute_pair(&k);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = zkp.solve(&k, &c, &x);
        assert_eq!(s, BigUint::from(5u32));

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);

        // fake secret
        let x_fake = BigUint::from(7u32);
        let s_fake = zkp.solve(&k, &c, &x_fake);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s_fake);
        assert!(!result);
    }

    #[test]
    fn test_toy_example_with_random_numbers() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = BigUint::from(6u32);
        let k = ZKP::generate_random_number_below(&q);

        let c = ZKP::generate_random_number_below(&q);

        let (y1, y2) = zkp.compute_pair(&x);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let (r1, r2) = zkp.compute_pair(&k);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[rustfmt::skip]
    #[test]
    fn test_1024_bits_constants()
    {
        //
        // Reference: https://www.rfc-editor.org/rfc/rfc5114#page-15
        //
        let p = BigUint::from_bytes_be(&hex::decode(
            "B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B6160\
             73E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCB\
             DD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF\
             5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap());

        let alpha = BigUint::from_bytes_be(&hex::decode(
            "A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D3126\
             6FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FB\
             D7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4\
             D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(),
        );

        // beta = alpha^i is also a generator
        let beta = alpha.modpow(&ZKP::generate_random_number_below(&q), &p);

        let zkp = ZKP::new(&p, &q, &alpha, &beta);

        let x = ZKP::generate_random_number_below(&q);
        let k = ZKP::generate_random_number_below(&q);

        let c = ZKP::generate_random_number_below(&q);

        let (y1, y2) = zkp.compute_pair(&x);

        let (r1, r2) = zkp.compute_pair(&k);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_2048_bits_constants() {
        //
        //    Reference: https://www.rfc-editor.org/rfc/rfc5114#page-15
        //
        //    The hexadecimal value of the prime is:
        //
        //    p =  AD107E1E 9123A9D0 D660FAA7 9559C51F A20D64E5 683B9FD1
        //         B54B1597 B61D0A75 E6FA141D F95A56DB AF9A3C40 7BA1DF15
        //         EB3D688A 309C180E 1DE6B85A 1274A0A6 6D3F8152 AD6AC212
        //         9037C9ED EFDA4DF8 D91E8FEF 55B7394B 7AD5B7D0 B6C12207
        //         C9F98D11 ED34DBF6 C6BA0B2C 8BBC27BE 6A00E0A0 B9C49708
        //         B3BF8A31 70918836 81286130 BC8985DB 1602E714 415D9330
        //         278273C7 DE31EFDC 7310F712 1FD5A074 15987D9A DC0A486D
        //         CDF93ACC 44328387 315D75E1 98C641A4 80CD86A1 B9E587E8
        //         BE60E69C C928B2B9 C52172E4 13042E9B 23F10B0E 16E79763
        //         C9B53DCF 4BA80A29 E3FB73C1 6B8E75B9 7EF363E2 FFA31F71
        //         CF9DE538 4E71B81C 0AC4DFFE 0C10E64F
        //
        //    The hexadecimal value of the generator is:
        //
        //    g =  AC4032EF 4F2D9AE3 9DF30B5C 8FFDAC50 6CDEBE7B 89998CAF
        //         74866A08 CFE4FFE3 A6824A4E 10B9A6F0 DD921F01 A70C4AFA
        //         AB739D77 00C29F52 C57DB17C 620A8652 BE5E9001 A8D66AD7
        //         C1766910 1999024A F4D02727 5AC1348B B8A762D0 521BC98A
        //         E2471504 22EA1ED4 09939D54 DA7460CD B5F6C6B2 50717CBE
        //         F180EB34 118E98D1 19529A45 D6F83456 6E3025E3 16A330EF
        //         BB77A86F 0C1AB15B 051AE3D4 28C8F8AC B70A8137 150B8EEB
        //         10E183ED D19963DD D9E263E4 770589EF 6AA21E7F 5F2FF381
        //         B539CCE3 409D13CD 566AFBB4 8D6C0191 81E1BCFE 94B30269
        //         EDFE72FE 9B6AA4BD 7B5A0F1C 71CFFF4C 19C418E1 F6EC0179
        //         81BC087F 2A7065B3 84B890D3 191F2BFA
        //
        //    The generator generates a prime-order subgroup of size:
        //
        //    q =  801C0D34 C58D93FE 99717710 1F80535A 4738CEBC BF389A99
        //         B36371EB
        //

        #[rustfmt::skip]
        let p = BigUint::from_bytes_be(&hex::decode(
            "AD107E1E9123A9D0D660FAA79559C51FA20D64E5683B9FD1B54B1597B61D0A75E6F\
             A141DF95A56DBAF9A3C407BA1DF15EB3D688A309C180E1DE6B85A1274A0A66D3F81\
             52AD6AC2129037C9EDEFDA4DF8D91E8FEF55B7394B7AD5B7D0B6C12207C9F98D11E\
             D34DBF6C6BA0B2C8BBC27BE6A00E0A0B9C49708B3BF8A317091883681286130BC89\
             85DB1602E714415D9330278273C7DE31EFDC7310F7121FD5A07415987D9ADC0A486\
             DCDF93ACC44328387315D75E198C641A480CD86A1B9E587E8BE60E69CC928B2B9C5\
             2172E413042E9B23F10B0E16E79763C9B53DCF4BA80A29E3FB73C16B8E75B97EF36\
             3E2FFA31F71CF9DE5384E71B81C0AC4DFFE0C10E64F").unwrap());

        let q = BigUint::from_bytes_be(
            &hex::decode("801C0D34C58D93FE997177101F80535A4738CEBCBF389A99B36371EB").unwrap(),
        );

        #[rustfmt::skip]
        let alpha = BigUint::from_bytes_be(
            &hex::decode(
                "AC4032EF4F2D9AE39DF30B5C8FFDAC506CDEBE7B89998CAF74866A08CFE4FFE\
                 3A6824A4E10B9A6F0DD921F01A70C4AFAAB739D7700C29F52C57DB17C620A86\
                 52BE5E9001A8D66AD7C17669101999024AF4D027275AC1348BB8A762D0521BC\
                 98AE247150422EA1ED409939D54DA7460CDB5F6C6B250717CBEF180EB34118E\
                 98D119529A45D6F834566E3025E316A330EFBB77A86F0C1AB15B051AE3D428C\
                 8F8ACB70A8137150B8EEB10E183EDD19963DDD9E263E4770589EF6AA21E7F5F\
                 2FF381B539CCE3409D13CD566AFBB48D6C019181E1BCFE94B30269EDFE72FE9\
                 B6AA4BD7B5A0F1C71CFFF4C19C418E1F6EC017981BC087F2A7065B384B890D3\
                 191F2BFA").unwrap(),
        );

        // beta = alpha^i is also a generator
        let beta = alpha.modpow(&ZKP::generate_random_number_below(&q), &p);

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = ZKP::generate_random_number_below(&q);
        let k = ZKP::generate_random_number_below(&q);

        let c = ZKP::generate_random_number_below(&q);

        let (y1, y2) = zkp.compute_pair(&x);
        let (r1, r2) = zkp.compute_pair(&k);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_invalid_inputs() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        let zkp = ZKP::new(&p, &q, &alpha, &beta);

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);
        let c = BigUint::from(4u32);

        let (y1, y2) = zkp.compute_pair(&x);
        let (r1, r2) = zkp.compute_pair(&k);
        let s = zkp.solve(&k, &c, &x);

        // Verify with an invalid c value
        let invalid_c = BigUint::from(999u32); // A value that's expected to fail verification
        let result = zkp.verify(&r1, &r2, &y1, &y2, &invalid_c, &s);
        assert!(!result); // Expect the verification to fail
    }
}
