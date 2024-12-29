/*

---- Model konstan yang harus ada ----

    Saya mendefinisikan model roman sampai 1000, dengan termasuk yang pengurangan angka 9 dan 4.
    Setelah itu tinggal lakukan perulangan untuk model tersebut. Simpel
    Maka kompleksitas tersebut harusnya adalah O(1), karena tidak peduli seberapa 
    besar inputnya, perulangannya tetep sama.

--------------------------------------

*/


pub fn int_to_roman(num: i32) -> String {
    // Define model 1000,900,500,400,100,90,50,40,10,9,5,4,1

    let model = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut result = String::new();
    let mut current_num = num;

    for (n, r) in model {
        let evaluated = r.repeat((current_num / n) as usize);
        current_num = current_num % n;
        result.push_str(&evaluated);
    }

    result
}