while inotifywait -q -r -e close_write ./ ../examples/
do
   clear
   clear
   RUST_BACKTRACE=1 cargo run testing.t
done
