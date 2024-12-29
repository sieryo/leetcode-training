/*

---- Lihat dengan perspektif yang berbeda ----

    Saya membuat variable yang untuk menampung nilai dari nilai komputasi yang sebelumnya untuk membandingkan
    apakah angka tersebut berfungsi sebagai pengurangan atau tidak. Mengapa harus before * 2 ? karena
    pertama kali angka pengurangan tersebut di-iterasi, otomatis before tidak valid karena variable tersebut
    harusnya lebih besar dari sekarang, yang membuat harus dijumlahkan lagi. Baru kalau current sudah lebih besar
    (artinya angka sebelumnya hanya sebagai pengurang) saya harus kuranginya menjadi double. O(n) mungkin?

----------------------------------------------

*/


pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;

    let mut before = 0;

    // Gimana kalau setiap iterasi ada variable sebelum? jadi fungsinya misal nih:
    // 1000 - 100 (CM), maka yang 100 itu dimasukin ke result, tapi kalau variable sebelumnya lebih kecil dari sekarang, maka yang sekarang akan dikurangi sebesar 2 kali lipat dari sebelum.

    for (_, r) in s.chars().enumerate() {
        let current = match r {
            'M' => 1000,
            'D' => 500,
            'C' => 100,
            'L' => 50,
            'X' => 10,
            'V' => 5,
            _ => 1,
        };

        if current > before {
            result = result + (current - before * 2);
        } else {
            result = result + current;
        }
        before = current;
    }

    result
}