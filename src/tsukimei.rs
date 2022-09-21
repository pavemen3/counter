use std::collections::HashMap;
fn main() {
  // 旧暦月名の一覧
  let tsuki = ["睦月","如月","弥生","卯月","皐月","水無月","文月","葉月","長月","神無月","霜月","師走"];

  // HashMapを初期化
  let mut tsuki_map: HashMap<&str, usize> = HashMap::new();
  // 月名をHashMapに追加
  for (i, v) in tsuki.iter().enumerate() {
    tsuki_map.insert(v, i+1);
  }
  // 要素を選んでひょうじ
  println!("水無月 = {}月", tsuki_map["水無月"]);
  println!("神無月 = {}月", tsuki_map["神無月"]);
  println!("師走 = {}月", tsuki_map["師走"]);
}

