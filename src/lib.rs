use num_bigint::{BigUint, RandBigInt};
use rand::Rng;

pub struct ZKP
{
    pub alpha: BigUint,
    pub beta:  BigUint,
    pub p:     BigUint,
    pub q:     BigUint,
}

impl ZKP
{
    pub fn new(alpha: &BigUint, beta: &BigUint, p: &BigUint, q: &BigUint) -> Self
    {
        Self {
            alpha: alpha.clone(),
            beta:  beta.clone(),
            p:     p.clone(),
            q:     q.clone(),
        }
    }

    /// output = (alpha^exp mod p, beta^exp mod p)
    pub fn compute_pair(&self, exp: &BigUint) -> (BigUint, BigUint)
    {
        let p1 = self.alpha.modpow(exp, &self.p);
        let p2 = self.beta.modpow(exp, &self.p);
        (p1, p2)
    }

    /// output = s = k - c * x mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint
    {
        if *k >= c * x
        {
            (k - c * x).modpow(&BigUint::from(1u32), &self.q)
        }
        else
        {
            &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q)
        }
    }

    /// cond1: r1 = alpha^s * y1^c
    /// cond2: r2 = beta^s * y2^c
    pub fn verify(
        &self, r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, c: &BigUint, s: &BigUint,
    ) -> bool
    {
        let cond1 = *r1
            == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        let cond2 = *r2
            == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        cond1 && cond2
    }

    pub fn generate_random_number_below(bound: &BigUint) -> BigUint
    {
        rand::thread_rng().gen_biguint_below(bound)
    }

    pub fn generate_random_string(size: usize) -> String
    {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }

    #[rustfmt::skip]
    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint)
    {
        // Reference: https://www.rfc-editor.org/rfc/rfc5114#page-15
        //
        let p = BigUint::from_bytes_be(&hex::decode(
            "B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B61607\
             3E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD\
             7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0\
             DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());

        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap());

        let alpha = BigUint::from_bytes_be(&hex::decode(
            "A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D3\
             1266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749\
             F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A\
             28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap());

        // beta = alpha^i is also a generator
        let beta = alpha.modpow(&ZKP::generate_random_number_below(&q), &p);

        (alpha, beta, p, q)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_toy_example()
    {
        let alpha = BigUint::from(4_u32);
        let beta = BigUint::from(9_u32);
        let p = BigUint::from(23_u32);
        let q = BigUint::from(11_u32);
        let zkp = ZKP::new(&alpha, &beta, &p, &q);

        let x = BigUint::from(6_u32);
        let k = BigUint::from(7_u32);

        let c = BigUint::from(4_u32);

        let (y1, y2) = zkp.compute_pair(&x);
        assert_eq!(y1, BigUint::from(2_u32));
        assert_eq!(y2, BigUint::from(3_u32));

        let (r1, r2) = zkp.compute_pair(&k);
        assert_eq!(r1, BigUint::from(8_u32));
        assert_eq!(r2, BigUint::from(4_u32));

        let s = zkp.solve(&k, &c, &x);
        assert_eq!(s, BigUint::from(5_u32));

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);

        // fake secret
        let x_fake = BigUint::from(7_u32);
        let s_fake = zkp.solve(&k, &c, &x_fake);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s_fake);
        assert!(!result);
    }

    #[test]
    fn test_toy_example_with_random_numbers()
    {
        let alpha = BigUint::from(4_u32);
        let beta = BigUint::from(9_u32);
        let p = BigUint::from(23_u32);
        let q = BigUint::from(11_u32);
        let zkp = ZKP::new(&alpha, &beta, &p, &q);

        let x = BigUint::from(6_u32);
        let k = ZKP::generate_random_number_below(&q);
        let c = ZKP::generate_random_number_below(&q);

        let (y1, y2) = zkp.compute_pair(&x);
        assert_eq!(y1, BigUint::from(2_u32));
        assert_eq!(y2, BigUint::from(3_u32));

        let (r1, r2) = zkp.compute_pair(&k);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_1024_bits_constants()
    {
        let (alpha, beta, p, q) = ZKP::get_constants();
        let zkp = ZKP::new(&alpha, &beta, &p, &q);

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
    fn test_2048_bits_constants()
    {
        //
        // Reference: https://www.rfc-editor.org/rfc/rfc5114#page-15

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
        let q = BigUint::from_bytes_be(&hex::decode(
            "801C0D34C58D93FE997177101F80535A4738CEBCBF389A99B36371EB").unwrap(),
        );

        let alpha = BigUint::from_bytes_be(&hex::decode(
            "AC4032EF4F2D9AE39DF30B5C8FFDAC506CDEBE7B89998CAF74866A08CFE4FFE3A68\
             24A4E10B9A6F0DD921F01A70C4AFAAB739D7700C29F52C57DB17C620A8652BE5E90\
             01A8D66AD7C17669101999024AF4D027275AC1348BB8A762D0521BC98AE24715042\
             2EA1ED409939D54DA7460CDB5F6C6B250717CBEF180EB34118E98D119529A45D6F8\
             34566E3025E316A330EFBB77A86F0C1AB15B051AE3D428C8F8ACB70A8137150B8EE\
             B10E183EDD19963DDD9E263E4770589EF6AA21E7F5F2FF381B539CCE3409D13CD56\
             6AFBB48D6C019181E1BCFE94B30269EDFE72FE9B6AA4BD7B5A0F1C71CFFF4C19C41\
             8E1F6EC017981BC087F2A7065B384B890D3191F2BFA").unwrap(),
        );

        // beta = alpha^i is also a generator
        let beta = alpha.modpow(&ZKP::generate_random_number_below(&q), &p);

        let zkp = ZKP::new(&alpha, &beta, &p, &q);

        let x = ZKP::generate_random_number_below(&q);
        let k = ZKP::generate_random_number_below(&q);
        let c = ZKP::generate_random_number_below(&q);

        let (y1, y2) = zkp.compute_pair(&x);
        let (r1, r2) = zkp.compute_pair(&k);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }
}
