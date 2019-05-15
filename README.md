Point of sale articles conversion tools
=======================================

Tools for converting General Bytes POS data to CSV and CSV to BTCPayServer template.

About
-----

These tools allow you to convert General Bytes POS articles into CSV (useful if you want to use spreadsheets etc.)
and convert CSV data to BTCPayServer template (Yaml format).

The tools are flexible, so you can convert the data also from CSV files that look different.

Usage
-----

These tools are written in Rust (I wanted to improve some of my Rust skills. :)), so you need Rust compiler installed.
See [Rustup](https://rustup.rs).

After compilation, man pages are generated - no need to look at the source code (but code review never hurts).

You just need to provide your input and output files. In case of `gbpos2csv` the input file is GB site with articles as HTML.

Example: migrating from General Bytes POS to BTCPayServer
---------------------------------------------------------

The migration is pretty easy with this tool:

```
cargo build
# While it's building, download the articles site as articles.html

./target/debug/gbpos2csv --input articles.html --output articles.csv --strip_currency
./target/debug/csv2btcpay --input articles.csv --output articles.yaml --price-field 4 --title-field 1 --prefix-field 2
# Copy-paste the content of articles.yaml as BTCPayServer POS template.
```
