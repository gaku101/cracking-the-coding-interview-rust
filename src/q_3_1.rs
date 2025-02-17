pub struct ThreeStacks<T> {
    data: Vec<Option<T>>,
    stack_size: usize,
    tops: [isize; 3], // 各スタックのトップのインデックス。-1 は空を意味する。
}

impl<T> ThreeStacks<T> {
    pub fn new(stack_size: usize) -> Self {
        let total_size = stack_size * 3;
        let mut data = Vec::with_capacity(total_size);
        data.resize_with(total_size, || None);
        Self {
            data,
            stack_size,
            tops: [-1; 3],
        }
    }

    pub fn push(&mut self, stack_num: usize, value: T) -> Result<(), &'static str> {
        if stack_num >= 3 {
            return Err("Invalid stack number");
        }
        if (self.tops[stack_num]) >= self.stack_size as isize - 1 {
            return Err("Stack Overflow");
        }

        self.tops[stack_num] += 1;
        let index = stack_num * self.stack_size + (self.tops[stack_num] as usize);
        self.data[index] = Some(value);
        Ok(())
    }

    pub fn pop(&mut self, stack_num: usize) -> Result<T, &'static str> {
        if stack_num >= 3 {
            return Err("Invalid stack number");
        }
        if self.tops[stack_num] < 0 {
            return Err("Stack Underflow");
        }
        let index = stack_num * self.stack_size + (self.tops[stack_num] as usize);
        let result = self.data[index].take().ok_or("Unexpected error")?;
        self.tops[stack_num] -= 1;
        Ok(result)
    }

    pub fn peek(&self, stack_num: usize) -> Result<&T, &'static str> {
        if stack_num >= 3 {
            return Err("Invalid stack number");
        }
        if self.tops[stack_num] < 0 {
            return Err("Stack is empty");
        }
        let index = stack_num * self.stack_size + (self.tops[stack_num] as usize);
        self.data[index].as_ref().ok_or("Unexpected error")
    }
}

/*
エラーメッセージとして文字列リテラル（例："Stack Overflow"）を返す場合、文字列リテラルはもともと 'static ライフタイムを持つので、戻り値の型を Result<T, &str> としても、ライフタイムは自動的に 'static と推論されるため、挙動やエラーは変わりません。

ただし、以下の点で違いがあります：

明示性

Result<T, &'static str> と明示すると、「エラー情報はプログラム全体で有効な静的な文字列である」という意図が明確になります。
ドキュメント的な意味では、将来的に動的なエラー文字列（必ずしも 'static でないもの）を扱う場合と区別できます。
汎用性の制限

もし、動的に生成された文字列スライス（たとえば、関数の引数などから借用している &str）を返す必要が出た場合、&'static str は使えなくなります。
一方で、Result<T, &str> としていると、ライフタイムパラメータが省略される形になり、コンパイラによって推論されるため、そのままでは使えないケースも発生するかもしれませんが、単に文字列リテラルだけを扱うなら影響はありません。
まとめると、文字列リテラルのみを扱うシンプルなエラー処理の場合、Result<T, &str> でも実質的な違いはないのでエラーも発生しません。ただ、意図を明示的に伝えるために、&'static str と書くケースが一般的です。
*/

/*
ライフタイムパラメータは、Rustで参照（&T や &mut T）の有効期間（寿命）を明示的に指定するための仕組みです。Rustの安全性は、所有権と借用のルールによって保証されていますが、ライフタイムパラメータはこれらのルールの一部として、参照がどのくらいの期間有効であるかをコンパイラに伝える役割を果たします。

ライフタイムパラメータの基本
目的
参照が有効な期間を静的に管理し、ダングリングポインタ（無効な参照）や二重解放などのメモリ安全性の問題を防ぐために用います。

構文
ライフタイムパラメータはシングルクォート（'）を先頭に付けて識別子を記述します。例えば、'a や 'static などがあります。

'static ライフタイム
文字列リテラルのように、プログラム全体を通じて有効な参照は 'static と呼ばれます。

関数でのライフタイムパラメータの例
よくある例として、2つの文字列スライスを受け取り、どちらか長い方の参照を返す関数があります。このとき、返り値の参照が引数のどちらに依存しているかを明示するためにライフタイムパラメータが必要になります。

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
ここでのポイントは以下の通りです:

<'a>
関数の定義でライフタイムパラメータ 'a を宣言しています。

s1: &'a str と s2: &'a str
どちらの引数も、ライフタイム 'a を持つ文字列スライスであると明示しています。
つまり、これらの参照は少なくとも同じ期間 'a だけ有効でなければなりません。

-> &'a str
戻り値も同じライフタイム 'a を持つ参照であることを示しています。
これにより、返される参照が、s1 または s2 のどちらかの有効期間を超えて利用されることがないとコンパイラが保証します。

ライフタイムパラメータが必要となる理由
Rustのコンパイラは、多くの場合ライフタイムを自動で推論してくれます。しかし、関数が複数の参照を扱う場合や、返り値のライフタイムが複数の引数のライフタイムのどちらに属するのか明確でない場合、明示的にライフタイムパラメータを書く必要があります。これにより、次のような点が保証されます:

安全性の保証
参照が有効な期間を明示することで、無効なメモリアクセス（ダングリングポインタ）を防ぎます。

コードの明確さ
関数や構造体の使用者に対して、どの参照がどの期間有効であるかが明示され、意図が明確になります。

構造体でのライフタイムパラメータの例
ライフタイムパラメータは構造体でも使用できます。例えば、参照をフィールドとして持つ構造体は以下のように定義します:

struct Book<'a> {
    title: &'a str,
    author: &'a str,
}
ここでは、Book 構造体の title と author フィールドは、すべて同じライフタイム 'a を持つ参照である必要があることを示しています。

まとめ
ライフタイムパラメータ は、参照の有効期間を静的に管理し、コンパイル時に安全性を保証するためのものです。
関数や構造体で複数の参照を扱う際、明示的にライフタイムパラメータを書くことで、返り値の参照がどの引数に依存するのか、またどの期間有効かを明確にします。
Rustの所有権システムと連携して、メモリ安全性を高めるための重要な仕組みです。
このように、ライフタイムパラメータはRustの安全性と正確なメモリ管理の根幹をなす重要な概念となります。
*/