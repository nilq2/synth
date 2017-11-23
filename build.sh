while inotifywait -q -r -e close_write ./ ../examples/
do
   clear
   clear
   RUST_BACKTRACE=1 cargo run tests/test.t tests/unit1.pi tests/unit2.pi
done
