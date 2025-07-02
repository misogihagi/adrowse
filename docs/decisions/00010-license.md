## 決定

ライセンスはMITかApache 2.0にする

## 根拠

APIガイドラインに従うため

https://rust-lang.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive

## 背景

> Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
> 
> Besides the dual MIT/Apache-2.0 license, another common licensing approach used by Rust crate authors is to apply a single permissive license such as MIT or BSD. This license scheme is also entirely compatible with Rust's, because it imposes the minimal restrictions of Rust's MIT license.
> 
> Crates that desire perfect license compatibility with Rust are not recommended to choose only the Apache license. The Apache license, though it is a permissive license, imposes restrictions  beyond the MIT and BSD licenses that can discourage or prevent their use in some scenarios, so Apache-only software cannot be used in some situations where most of the Rust runtime stack can.
> 
> The license of a crate's dependencies can affect the restrictions on distribution of the crate itself, so a permissively-licensed crate should generally only depend on permissively-licensed crates.


> あなたが明示的に別段の定めをしない限り、あなたが作品に含めるために意図的に提出した貢献は、Apache-2.0ライセンスで定義されているように、上記のようにデュアルライセンスされ、いかなる追加条項や条件もないものとします。
> 
> MIT/Apache-2.0ライセンスの二重ライセンスの他に、Rustクレートの作者によって、MITやBSDのような単一の寛容なライセンスを適用することも、一般的なライセンスアプローチです。、RustのMITライセンスの最小限の制限を課すため、このライセンススキームもRustと完全に互換性があります。
> 
> Rustとの完全なライセンス互換性を望むクレートは、Apacheライセンスのみを選択することは推奨されません。Apacheライセンスは、寛容なライセンスではありますが、MITライセンスやBSDライセンス以上の制限を課しているため、シナリオによっては使用を抑制したり、妨げたりすることがあります。そのため、Apacheのみのソフトウェア、Rustランタイムスタックのほとんどが使用できる状況でも使用できないことがあります。
> 
> クレートの依存関係のライセンスは、クレート自体の配布の制限に影響する可能性があるので、パーミッシブライセンスのクレートは、一般的にパーミッシブライセンスのクレートにのみ依存する必要があります。
