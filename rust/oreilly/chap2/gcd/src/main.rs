use std::io::Write;
use std::str::FromStr;

// main関数は値を返さないので、返り値の型の記述はしない
fn main () {
    gcd();
}

// コマンドライン引数からsimple_gcdを実行する関数
fn gcd () {
    // mutでないと値を代入できない
    // vectorに入れる型を指定する必要はない。推論してくれる
    let mut numbers = Vec::new();

    // コマンドライン引数から値をベクトルに格納
    // std::env::argsはイテレータを返す
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    // 引数がなかったら、標準エラーストリームにエラーメッセージを書き出す
    // unwrap:エラーメッセージの出力の成功確認
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Useage: gcd Number ...").unwrap();
        std::process::exit(1);
    }

    // vectorのサイズは任意なので、とても大きくなる可能性がある
    // rustはこういった値を慎重に扱う
    // 所有権はnumbersにあって、ループでは借用しているだけ。
    // &で要素への参照、*で参照解決
    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = simple_gcd(d, *m)
    }
    println!("The greatest commpn divisor of{:?} is {}", numbers, d);
}

// シンプルにユークリッドのアルゴリズムで最大公約数を求める
// gcd(A, B)で、小さい方の数で大きい方を割った余りを次の数に回してどんどん割る
fn simple_gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(simple_gcd(14, 15), 1);
    assert_eq!(simple_gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
